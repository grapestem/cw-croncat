use crate::error::ContractError;
use crate::helpers::{send_tokens, GenericBalance};
use crate::state::{Config, CwCroncat};
use cosmwasm_std::{
    has_coins, Addr, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, SubMsg,
};
use cw20::Balance;
use std::ops::Div;

use cw_croncat_core::msg::{GetAgentIdsResponse, GetAgentTasksResponse};
use cw_croncat_core::types::{Agent, AgentStatus};

impl<'a> CwCroncat<'a> {
    /// Get a single agent details
    /// Check's status as well, in case this agent needs to be considered for election
    pub(crate) fn query_get_agent(&self, deps: Deps, account_id: Addr) -> StdResult<Option<Agent>> {
        let agent = self.agents.may_load(deps.storage, account_id.clone())?;
        if agent.is_none() {
            return Ok(None);
        }
        let a = agent.unwrap();

        let pending: Vec<Addr> = self
            .agent_pending_queue
            .may_load(deps.storage)?
            .unwrap_or_default();

        // If agent is pending, Check if they should get nominated to checkin to become active
        let agent_status: AgentStatus = if a.status == AgentStatus::Pending {
            // TODO: change to check total tasks + task ratio
            if pending.contains(&account_id) {
                AgentStatus::Nominated
            } else {
                a.status
            }
        } else {
            a.status
        };

        Ok(Some(Agent {
            status: agent_status,
            payable_account_id: a.payable_account_id,
            balance: a.balance,
            total_tasks_executed: a.total_tasks_executed,
            last_missed_slot: a.last_missed_slot,
            register_start: a.register_start,
        }))
    }

    /// Get a list of agent addresses
    pub(crate) fn query_get_agent_ids(&self, deps: Deps) -> StdResult<GetAgentIdsResponse> {
        // let active = self.agent_active_queue.load(deps.storage)?;
        // let pending = self.agent_pending_queue.load(deps.storage)?;
        let active: Vec<Addr> = self
            .agent_active_queue
            .may_load(deps.storage)?
            .unwrap_or_default();
        let pending: Vec<Addr> = self
            .agent_pending_queue
            .may_load(deps.storage)?
            .unwrap_or_default();

        Ok(GetAgentIdsResponse { active, pending })
        // Ok(GetAgentIdsResponse(active, pending))
    }

    // TODO:
    /// Check how many tasks an agent can execute
    pub(crate) fn query_get_agent_tasks(
        &self,
        _deps: Deps,
        _account_id: Addr,
    ) -> StdResult<GetAgentTasksResponse> {
        // let active = self.agent_active_queue.load(deps.storage)?;

        Ok(GetAgentTasksResponse(0, 0))
    }

    /// Add any account as an agent that will be able to execute tasks.
    /// Registering allows for rewards accruing with micro-payments which will accumulate to more long-term.
    ///
    /// Optional Parameters:
    /// "payable_account_id" - Allows a different account id to be specified, so a user can receive funds at a different account than the agent account.
    pub fn register_agent(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        payable_account_id: Option<Addr>,
    ) -> Result<Response, ContractError> {
        if !info.funds.is_empty() {
            return Err(ContractError::CustomError {
                val: "Do not attach funds".to_string(),
            });
        }
        let c: Config = self.config.load(deps.storage)?;
        if c.paused {
            return Err(ContractError::CustomError {
                val: "Register agent paused".to_string(),
            });
        }

        let account = info.sender;

        // REF: https://github.com/CosmWasm/cw-tokens/tree/main/contracts/cw20-escrow
        // Check if native token balance is sufficient for a few txns, in this case 4 txns
        // TODO: Adjust gas & costs based on real usage cost
        let agent_wallet_balances = deps.querier.query_all_balances(account.clone())?;
        let unit_cost = c.gas_price * 4;
        if !has_coins(
            &agent_wallet_balances,
            &Coin::new(u128::from(unit_cost), c.native_denom),
        ) || agent_wallet_balances.is_empty()
        {
            return Err(ContractError::CustomError {
                val: "Insufficient funds".to_string(),
            });
        }

        let payable_id = payable_account_id.unwrap_or_else(|| account.clone());

        let mut active_agents: Vec<Addr> = self
            .agent_active_queue
            .may_load(deps.storage)?
            .unwrap_or_default();
        let total_agents = active_agents.len();
        let agent_status = if total_agents == 0 {
            active_agents.push(account.clone());
            self.agent_active_queue.save(deps.storage, &active_agents)?;
            AgentStatus::Active
        } else {
            let mut pending_agents = self
                .agent_pending_queue
                .may_load(deps.storage)?
                .unwrap_or_default();
            pending_agents.push(account.clone());
            self.agent_pending_queue
                .save(deps.storage, &pending_agents)?;
            AgentStatus::Pending
        };

        self.agents.update(
            deps.storage,
            account,
            |a: Option<Agent>| -> Result<_, ContractError> {
                match a {
                    // make sure that account isn't already added
                    Some(_) => Err(ContractError::CustomError {
                        val: "Agent already exists".to_string(),
                    }),
                    None => {
                        Ok(Agent {
                            status: agent_status.clone(),
                            payable_account_id: payable_id,
                            balance: GenericBalance::default(),
                            total_tasks_executed: 0,
                            last_missed_slot: 0,
                            // REF: https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/types.rs#L57
                            register_start: env.block.time.nanos(),
                        })
                    }
                }
            },
        )?;

        Ok(Response::new()
            .add_attribute("method", "register_agent")
            .add_attribute("agent_status", format!("{:?}", agent_status))
            .add_attribute("register_start", env.block.time.nanos().to_string()))
    }

    /// Update agent details, specifically the payable account id for an agent.
    pub fn update_agent(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        _env: Env,
        payable_account_id: Addr,
    ) -> Result<Response, ContractError> {
        let c: Config = self.config.load(deps.storage)?;
        if c.paused {
            return Err(ContractError::CustomError {
                val: "Register agent paused".to_string(),
            });
        }

        self.agents.update(
            deps.storage,
            info.sender,
            |a: Option<Agent>| -> Result<_, ContractError> {
                match a {
                    Some(agent) => {
                        let mut ag = agent;
                        ag.payable_account_id = payable_account_id;
                        Ok(ag)
                    }
                    None => Err(ContractError::CustomError {
                        val: "Agent doesnt exist".to_string(),
                    }),
                }
            },
        )?;

        Ok(Response::new().add_attribute("method", "update_agent"))
    }

    /// Allows an agent to withdraw all rewards, paid to the specified payable account id.
    pub(crate) fn withdraw_balances(
        &self,
        storage: &mut dyn Storage,
        info: MessageInfo,
    ) -> Result<Vec<SubMsg>, ContractError> {
        let a = self.agents.may_load(storage, info.sender)?;
        if a.is_none() {
            return Err(ContractError::CustomError {
                val: "Agent doesnt exist".to_string(),
            });
        }
        let agent = a.unwrap();

        // This will send all token balances to Agent
        let (messages, balances) = send_tokens(&agent.payable_account_id, &agent.balance)?;
        let mut config = self.config.load(storage)?;
        config
            .available_balance
            .minus_tokens(Balance::from(balances.native));
        // TODO: Finish:
        // config
        //     .available_balance
        //     .minus_tokens(Balance::from(balances.cw20));
        self.config.save(storage, &config)?;

        Ok(messages)
    }

    /// Allows an agent to withdraw all rewards, paid to the specified payable account id.
    pub fn withdraw_task_balance(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        _env: Env,
    ) -> Result<Response, ContractError> {
        let messages = self.withdraw_balances(deps.storage, info.clone())?;

        Ok(Response::new()
            .add_attribute("method", "withdraw_task_balance")
            .add_attribute("account_id", info.sender)
            .add_submessages(messages))
    }

    /// Allows an agent to accept a nomination within a certain amount of time to become an active agent.
    pub fn accept_nomination_agent(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
    ) -> Result<Response, ContractError> {
        // Compare current time and Config's agent_nomination_begin_time to see if agent can join
        let mut c: Config = self.config.load(deps.storage)?;

        let time_difference = if let Some(nomination_start) = c.agent_nomination_begin_time {
            env.block.time.seconds() - nomination_start.seconds()
        } else {
            // No agents can join yet
            return Err(ContractError::CustomError {
                val: "Not accepting new agents".to_string(),
            });
        };
        // Agent must be in the pending queue
        let pending_queue = self.agent_pending_queue.may_load(deps.storage)?.unwrap();
        // Get the position in the pending queue
        if let Some(agent_position) = pending_queue
            .iter()
            .position(|address| address == &info.sender)
        {
            // It works out such that the time difference between when this is called,
            // and the agent nomination begin time can be divided by the nomination
            // duration and we get an integer. We use that integer to determine if an
            // agent is allowed to get let in. If their position in the pending queue is
            // less than or equal to that integer, they get let in.
            let max_index = time_difference.div(c.agent_nomination_duration as u64);
            if agent_position as u64 <= max_index {
                // They're allowed in, update state for agents map
                // TODO: remove this and the idea of Agent having the status
                self.agents.update(
                    deps.storage,
                    info.sender.clone(),
                    |a: Option<Agent>| -> Result<_, ContractError> {
                        match a {
                            // make sure that account isn't already added
                            Some(agent) => Ok(Agent {
                                status: AgentStatus::Active,
                                payable_account_id: agent.payable_account_id,
                                balance: agent.balance,
                                total_tasks_executed: agent.total_tasks_executed,
                                last_missed_slot: agent.last_missed_slot,
                                register_start: agent.register_start,
                            }),
                            None => Err(ContractError::CustomError {
                                val: "Agent already exists".to_string(),
                            }),
                        }
                    },
                )?;
                // Also update state removing from pending queue
                let mut pending_agents: Vec<Addr> = self
                    .agent_pending_queue
                    .may_load(deps.storage)?
                    .unwrap_or_default();

                // Remove this agent and all ahead of them in the queue (they missed out)
                for idx_to_remove in (0..=agent_position).rev() {
                    println!("Removing pending index {}", idx_to_remove);
                    pending_agents.remove(idx_to_remove);
                }

                self.agent_pending_queue
                    .save(deps.storage, &pending_agents)?;
                // and adding to active queue
                let mut active_agents: Vec<Addr> = self
                    .agent_active_queue
                    .may_load(deps.storage)?
                    .unwrap_or_default();
                active_agents.push(info.sender.clone());
                self.agent_active_queue.save(deps.storage, &active_agents)?;
                // and update the config, setting the nomination begin time to None,
                // which indicates no one will be nominated until more tasks arrive
                c.agent_nomination_begin_time = None;
                self.config.save(deps.storage, &c)?;
            } else {
                return Err(ContractError::CustomError {
                    val: "Must wait longer before accepting nomination".to_string(),
                });
            }
        } else {
            // Sender's address does not exist in the agent pending queue
            return Err(ContractError::CustomError {
                val: "Sender is not a pending agent".to_string(),
            });
        }
        // Find difference
        Ok(Response::new().add_attribute("method", "accept_nomination_agent"))
    }

    /// Removes the agent from the active set of agents.
    /// Withdraws all reward balances to the agent payable account id.
    pub fn unregister_agent(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        _env: Env,
    ) -> Result<Response, ContractError> {
        // Get withdraw messages, if any
        // NOTE: Since this also checks if agent exists, safe to not have redundant logic
        let messages = self.withdraw_balances(deps.storage, info.clone())?;
        let agent_id = info.sender;
        self.agents.remove(deps.storage, agent_id.clone());

        let responses = Response::new()
            .add_attribute("method", "unregister_agent")
            .add_attribute("account_id", agent_id);

        if messages.is_empty() {
            Ok(responses)
        } else {
            Ok(responses.add_submessages(messages))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ContractError;
    use crate::helpers::CwTemplateContract;
    use cosmwasm_std::testing::mock_env;
    use cosmwasm_std::{
        coin, coins, from_binary, Addr, BlockInfo, CosmosMsg, Empty, StakingMsg, Timestamp,
    };
    use cw_croncat_core::msg::{
        ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TaskRequest, TaskResponse,
    };
    use cw_croncat_core::types::{Boundary, Interval};
    use cw_multi_test::{App, AppBuilder, AppResponse, Contract, ContractWrapper, Executor};
    use schemars::_private::NoSerialize;
    use std::convert::TryInto;
    use std::error::Error;

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::entry::execute,
            crate::entry::instantiate,
            crate::entry::query,
        );
        Box::new(contract)
    }

    const AGENT0: &str = "cosmos1a7uhnpqthunr2rzj0ww0hwurpn42wyun6c5puz";
    const AGENT1: &str = "cosmos17muvdgkep4ndptnyg38eufxsssq8jr3wnkysy8";
    const AGENT2: &str = "cosmos1qxywje86amll9ptzxmla5ah52uvsd9f7drs2dl";
    const AGENT3: &str = "cosmos1c3cy3wzzz3698ypklvh7shksvmefj69xhm89z2";
    const AGENT4: &str = "cosmos1ykfcyj8fl6xzs88tsls05x93gmq68a7km05m4j";
    const AGENT_BENEFICIARY: &str = "cosmos1t5u0jfg3ljsjrh2m9e47d4ny2hea7eehxrzdgd";
    const ADMIN: &str = "cosmos1sjllsnramtg3ewxqwwrwjxfgc4n4ef9u0tvx7u";
    const PARTICIPANT0: &str = "cosmos1055rfv3fv0zxsp8h3x88mctnm7x9mlgmf4m4d6";
    const PARTICIPANT1: &str = "cosmos1c3cy3wzzz3698ypklvh7shksvmefj69xhm89z2";
    const PARTICIPANT2: &str = "cosmos1far5cqkvny7k9wq53aw0k42v3f76rcylzzv05n";
    const PARTICIPANT3: &str = "cosmos1xj3xagnprtqpfnvyp7k393kmes73rpuxqgamd8";
    const PARTICIPANT4: &str = "cosmos1t5u0jfg3ljsjrh2m9e47d4ny2hea7eehxrzdgd";
    const NATIVE_DENOM: &str = "atom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            let accounts: Vec<(u128, String)> = vec![
                (100, ADMIN.to_string()),
                (1, AGENT0.to_string()),
                (100, AGENT1.to_string()),
                (100, AGENT2.to_string()),
                (100, AGENT3.to_string()),
                (100, AGENT4.to_string()),
                (20, PARTICIPANT0.to_string()),
                (20, PARTICIPANT1.to_string()),
                (20, PARTICIPANT2.to_string()),
                (20, PARTICIPANT3.to_string()),
                (20, PARTICIPANT4.to_string()),
                (1, AGENT_BENEFICIARY.to_string()),
            ];
            for (amt, address) in accounts.iter() {
                router
                    .bank
                    .init_balance(
                        storage,
                        &Addr::unchecked(address),
                        vec![coin(amt.clone(), NATIVE_DENOM.to_string())],
                    )
                    .unwrap();
            }
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());
        let owner_addr = Addr::unchecked(ADMIN);

        let msg = InstantiateMsg {
            denom: "atom".to_string(),
            owner_id: Some(owner_addr.clone()),
            agent_nomination_duration: 360,
        };
        let cw_template_contract_addr = app
            .instantiate_contract(cw_template_id, owner_addr, &msg, &[], "Manager", None)
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    fn get_task_total(app: &App, contract_addr: &Addr) -> usize {
        let res: Vec<TaskResponse> = app
            .wrap()
            .query_wasm_smart(
                contract_addr,
                &QueryMsg::GetTasks {
                    from_index: None,
                    limit: None,
                },
            )
            .unwrap();
        res.len()
    }

    fn add_task_exec(app: &mut App, contract_addr: &Addr, sender: &str) -> AppResponse {
        let validator = String::from("you");
        let amount = coin(3, NATIVE_DENOM);
        let stake = StakingMsg::Delegate { validator, amount };
        let msg: CosmosMsg = stake.clone().into();
        let send_funds = coins(1, NATIVE_DENOM);
        let hi = app.execute_contract(
            Addr::unchecked(sender),
            contract_addr.clone(),
            &ExecuteMsg::CreateTask {
                task: TaskRequest {
                    interval: Interval::Immediate,
                    boundary: Boundary {
                        start: None,
                        end: None,
                    },
                    stop_on_fail: false,
                    action: msg,
                    rules: None,
                },
            },
            send_funds.as_ref(),
        );
        hi.expect("Error adding task")
    }

    fn register_agent_exec(
        app: &mut App,
        contract_addr: &Addr,
        agent: &str,
        beneficiary: &str,
    ) -> AppResponse {
        app.execute_contract(
            Addr::unchecked(agent),
            contract_addr.clone(),
            &ExecuteMsg::RegisterAgent {
                payable_account_id: Some(Addr::unchecked(beneficiary)),
            },
            &[],
        )
        .expect("Error registering agent")
    }

    fn check_in_exec(
        app: &mut App,
        contract_addr: &Addr,
        agent: &str,
    ) -> Result<AppResponse, anyhow::Error> {
        app.execute_contract(
            Addr::unchecked(agent),
            contract_addr.clone(),
            &ExecuteMsg::CheckInAgent {},
            &[],
        )
    }

    fn get_agent_ids(app: &App, contract_addr: &Addr) -> (GetAgentIdsResponse, usize, usize) {
        let res: GetAgentIdsResponse = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::GetAgentIds {})
            .unwrap();
        (res.clone(), res.active.len(), res.pending.len())
    }

    pub fn add_little_time(block: &mut BlockInfo) {
        // block.time = block.time.plus_seconds(360);
        block.time = block.time.plus_seconds(19);
        block.height += 1;
    }

    pub fn add_one_duration_of_time(block: &mut BlockInfo) {
        // block.time = block.time.plus_seconds(360);
        block.time = block.time.plus_seconds(420);
        block.height += 1;
    }

    #[test]
    fn register_agent_fail_cases() {
        let (mut app, cw_template_contract) = proper_instantiate();
        let contract_addr = cw_template_contract.addr();

        // start first register
        let msg = ExecuteMsg::RegisterAgent {
            payable_account_id: Some(Addr::unchecked(AGENT_BENEFICIARY)),
        };

        // Test funds fail register if sent
        let rereg_err = app
            .execute_contract(
                Addr::unchecked(AGENT1),
                contract_addr.clone(),
                &msg,
                &coins(37, "atom"),
            )
            .unwrap_err();
        assert_eq!(
            ContractError::CustomError {
                val: "Do not attach funds".to_string()
            },
            rereg_err.downcast().unwrap()
        );

        // Test Can't register if contract is paused
        let payload_1 = ExecuteMsg::UpdateSettings {
            paused: Some(true),
            owner_id: None,
            // treasury_id: None,
            agent_fee: None,
            agent_task_ratio: None,
            agents_eject_threshold: None,
            gas_price: None,
            proxy_callback_gas: None,
            slot_granularity: None,
        };

        app.execute_contract(
            Addr::unchecked(ADMIN),
            contract_addr.clone(),
            &payload_1,
            &[],
        )
        .unwrap();
        let rereg_err = app
            .execute_contract(Addr::unchecked(AGENT1), contract_addr.clone(), &msg, &[])
            .unwrap_err();
        assert_eq!(
            ContractError::CustomError {
                val: "Register agent paused".to_string()
            },
            rereg_err.downcast().unwrap()
        );

        // Test wallet rejected if doesnt have enough funds
        let payload_2 = ExecuteMsg::UpdateSettings {
            paused: Some(false),
            owner_id: None,
            // treasury_id: None,
            agent_fee: None,
            agent_task_ratio: None,
            agents_eject_threshold: None,
            gas_price: None,
            proxy_callback_gas: None,
            slot_granularity: None,
        };

        app.execute_contract(
            Addr::unchecked(ADMIN),
            contract_addr.clone(),
            &payload_2,
            &[],
        )
        .unwrap();
        let rereg_err = app
            .execute_contract(Addr::unchecked(AGENT0), contract_addr.clone(), &msg, &[])
            .unwrap_err();
        assert_eq!(
            ContractError::CustomError {
                val: "Insufficient funds".to_string()
            },
            rereg_err.downcast().unwrap()
        );
    }

    #[test]
    fn register_agent() {
        let (mut app, cw_template_contract) = proper_instantiate();
        let contract_addr = cw_template_contract.addr();
        let blk_time = app.block_info().time;

        // start first register
        let msg = ExecuteMsg::RegisterAgent {
            payable_account_id: Some(Addr::unchecked(AGENT_BENEFICIARY)),
        };
        app.execute_contract(Addr::unchecked(AGENT1), contract_addr.clone(), &msg, &[])
            .unwrap();

        // check state to see if worked
        let (agent_ids_res, num_active_agents, num_pending_agents) =
            get_agent_ids(&app, &contract_addr);
        assert_eq!(1, num_active_agents);
        assert_eq!(0, num_pending_agents);

        // message response matches expectations (same block, all the defaults)
        let agent_info: Agent = app
            .wrap()
            .query_wasm_smart(
                &contract_addr.clone(),
                &QueryMsg::GetAgent {
                    account_id: Addr::unchecked(AGENT1),
                },
            )
            .unwrap();
        println!("agent_infoagent_info {:?}", agent_info);
        assert_eq!(AgentStatus::Active, agent_info.status);
        assert_eq!(
            Addr::unchecked(AGENT_BENEFICIARY),
            agent_info.payable_account_id
        );
        assert_eq!(GenericBalance::default(), agent_info.balance);
        assert_eq!(0, agent_info.total_tasks_executed);
        assert_eq!(0, agent_info.last_missed_slot);
        assert_eq!(blk_time, Timestamp::from_nanos(agent_info.register_start));

        // test fail if try to re-register
        let rereg_err = app
            .execute_contract(Addr::unchecked(AGENT1), contract_addr.clone(), &msg, &[])
            .unwrap_err();
        assert_eq!(
            ContractError::CustomError {
                val: "Agent already exists".to_string()
            },
            rereg_err.downcast().unwrap()
        );

        // test another register, put into pending queue
        let msg2 = ExecuteMsg::RegisterAgent {
            payable_account_id: Some(Addr::unchecked(AGENT_BENEFICIARY)),
        };
        app.execute_contract(Addr::unchecked(AGENT2), contract_addr.clone(), &msg2, &[])
            .unwrap();

        // check state to see if worked

        let (_, num_active_agents, num_pending_agents) = get_agent_ids(&app, &contract_addr);
        assert_eq!(1, num_active_agents);
        assert_eq!(1, num_pending_agents);
    }

    #[test]
    fn update_agent() {
        let (mut app, cw_template_contract) = proper_instantiate();
        let contract_addr = cw_template_contract.addr();

        // start first register
        let msg1 = ExecuteMsg::RegisterAgent {
            payable_account_id: Some(Addr::unchecked(AGENT_BENEFICIARY)),
        };
        app.execute_contract(Addr::unchecked(AGENT1), contract_addr.clone(), &msg1, &[])
            .unwrap();

        // Fails for non-existent agents
        let msg = ExecuteMsg::UpdateAgent {
            payable_account_id: Addr::unchecked(AGENT0),
        };
        let update_err = app
            .execute_contract(Addr::unchecked(AGENT0), contract_addr.clone(), &msg, &[])
            .unwrap_err();
        assert_eq!(
            ContractError::CustomError {
                val: "Agent doesnt exist".to_string()
            },
            update_err.downcast().unwrap()
        );

        app.execute_contract(Addr::unchecked(AGENT1), contract_addr.clone(), &msg, &[])
            .unwrap();

        // payable account was in fact updated
        let agent_info: Agent = app
            .wrap()
            .query_wasm_smart(
                &contract_addr.clone(),
                &QueryMsg::GetAgent {
                    account_id: Addr::unchecked(AGENT1),
                },
            )
            .unwrap();
        assert_eq!(Addr::unchecked(AGENT0), agent_info.payable_account_id);
    }

    #[test]
    fn unregister_agent() {
        let (mut app, cw_template_contract) = proper_instantiate();
        let contract_addr = cw_template_contract.addr();

        // start first register
        let msg1 = ExecuteMsg::RegisterAgent {
            payable_account_id: Some(Addr::unchecked(AGENT_BENEFICIARY)),
        };
        app.execute_contract(Addr::unchecked(AGENT1), contract_addr.clone(), &msg1, &[])
            .unwrap();

        // Fails for non-exist agents
        let unreg_msg = ExecuteMsg::UnregisterAgent {};
        let update_err = app
            .execute_contract(
                Addr::unchecked(AGENT0),
                contract_addr.clone(),
                &unreg_msg,
                &[],
            )
            .unwrap_err();
        assert_eq!(
            ContractError::CustomError {
                val: "Agent doesnt exist".to_string()
            },
            update_err.downcast().unwrap()
        );

        // Get quick data about account before, to compare later
        let agent_bal = app
            .wrap()
            .query_balance(&Addr::unchecked(AGENT1), NATIVE_DENOM)
            .unwrap();
        assert_eq!(agent_bal, coin(100, NATIVE_DENOM));

        // Attempt the unregister
        app.execute_contract(
            Addr::unchecked(AGENT1),
            contract_addr.clone(),
            &unreg_msg,
            &[],
        )
        .unwrap();

        // Agent should not exist now
        let update_err = app
            .execute_contract(
                Addr::unchecked(AGENT1),
                contract_addr.clone(),
                &unreg_msg,
                &[],
            )
            .unwrap_err();
        assert_eq!(
            ContractError::CustomError {
                val: "Agent doesnt exist".to_string()
            },
            update_err.downcast().unwrap()
        );

        // Agent should have appropriate balance change
        // NOTE: Needs further checks when tasks can be performed
        let agent_bal = app
            .wrap()
            .query_balance(&Addr::unchecked(AGENT1), NATIVE_DENOM)
            .unwrap();
        assert_eq!(agent_bal, coin(100, NATIVE_DENOM));
    }

    #[test]
    fn withdraw_task_balance() {
        let (mut app, cw_template_contract) = proper_instantiate();
        let contract_addr = cw_template_contract.addr();

        // start first register
        let msg1 = ExecuteMsg::RegisterAgent {
            payable_account_id: Some(Addr::unchecked(AGENT_BENEFICIARY)),
        };
        app.execute_contract(Addr::unchecked(AGENT1), contract_addr.clone(), &msg1, &[])
            .unwrap();

        // Fails for non-existent agents
        let wthdrw_msg = ExecuteMsg::WithdrawReward {};
        let update_err = app
            .execute_contract(
                Addr::unchecked(AGENT0),
                contract_addr.clone(),
                &wthdrw_msg,
                &[],
            )
            .unwrap_err();
        assert_eq!(
            ContractError::CustomError {
                val: "Agent doesnt exist".to_string()
            },
            update_err.downcast().unwrap()
        );

        // Get quick data about account before, to compare later
        let agent_bal = app
            .wrap()
            .query_balance(&Addr::unchecked(AGENT1), NATIVE_DENOM)
            .unwrap();
        assert_eq!(agent_bal, coin(100, NATIVE_DENOM));

        // Attempt the withdraw
        app.execute_contract(
            Addr::unchecked(AGENT1),
            contract_addr.clone(),
            &wthdrw_msg,
            &[],
        )
        .unwrap();

        // Agent should have appropriate balance change
        // NOTE: Needs further checks when tasks can be performed
        let agent_bal = app
            .wrap()
            .query_balance(&Addr::unchecked(AGENT1), NATIVE_DENOM)
            .unwrap();
        assert_eq!(agent_bal, coin(100, NATIVE_DENOM));
    }

    #[test]
    fn accept_nomination_agent() {
        let (mut app, cw_template_contract) = proper_instantiate();
        let contract_addr = cw_template_contract.addr();
        let mut deps = cosmwasm_std::testing::mock_dependencies_with_balance(&coins(200, ""));

        // Register AGENT1, who immediately becomes active
        register_agent_exec(&mut app, &contract_addr, AGENT1, &AGENT_BENEFICIARY);
        let mut res = add_task_exec(&mut app, &contract_addr, PARTICIPANT0);
        let task_hash = res.events[1].attributes[2].clone().value;
        assert_eq!(
            "fb4839c3fb825b0927201d3966544b85b233b0f236fd073c78d861e10a39e475", task_hash,
            "Unexpected task hash"
        );

        let msg_query_task = QueryMsg::GetTask { task_hash };
        let query_task_res: StdResult<Option<TaskResponse>> = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &msg_query_task);
        assert!(
            query_task_res.is_ok(),
            "Did not successfully find the newly added task"
        );

        // Get config object, which has the agent to task ratio we want to get
        let msg_query_config = QueryMsg::GetConfig {};
        let mut query_config: StdResult<ConfigResponse> = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &msg_query_config);
        let task_ratio = query_config.unwrap().agent_task_ratio;

        let mut num_tasks = get_task_total(&app, &contract_addr);
        assert_eq!(num_tasks, 1);

        // Now the task ratio is 1:2 (one agent per two tasks)
        // No agent should be allowed to join or accept nomination
        // Check that this fails

        // Register two agents
        register_agent_exec(&mut app, &contract_addr, AGENT2, &AGENT_BENEFICIARY);
        // Later, we'll have this agent try to nominate themselves before their time
        register_agent_exec(&mut app, &contract_addr, AGENT3, &AGENT_BENEFICIARY);

        let (mut agent_ids_res, mut num_active_agents, mut num_pending_agents) =
            get_agent_ids(&app, &contract_addr);
        assert_eq!(1, num_active_agents);
        assert_eq!(2, agent_ids_res.pending.len());

        // Add two more tasks, so we can nominate another agent
        add_task_exec(&mut app, &contract_addr, PARTICIPANT1);
        add_task_exec(&mut app, &contract_addr, PARTICIPANT2);

        num_tasks = get_task_total(&app, &contract_addr);
        assert_eq!(num_tasks, 3);

        // Fast forward time a little
        app.update_block(add_little_time);

        // Attempt to accept nomination
        // First try with the agent second in line in the pending queue.
        // This should fail because it's not time for them yet.
        let mut check_in_res = check_in_exec(&mut app, &contract_addr, AGENT3);
        assert!(
            &check_in_res.is_err(),
            "Should throw error when agent in second position tries to nominate before their time."
        );
        assert_eq!(
            ContractError::CustomError {
                val: "Must wait longer before accepting nomination".to_string()
            },
            check_in_res.unwrap_err().downcast().unwrap()
        );

        // Now try from person at the beginning of the pending queue
        // This agent should succeed
        check_in_res = check_in_exec(&mut app, &contract_addr, AGENT2);
        assert!(
            check_in_res.is_ok(),
            "Agent at the front of the pending queue should be allowed to nominate themselves"
        );

        // Check that active and pending queues are correct
        let (agent_ids_res, num_active_agents, num_pending_agents) =
            get_agent_ids(&app, &contract_addr);
        assert_eq!(2, num_active_agents);
        assert_eq!(1, agent_ids_res.pending.len());

        // The agent that was second in the queue is now first,
        // tries again, but there aren't enough tasks
        check_in_res = check_in_exec(&mut app, &contract_addr, AGENT3);
        let (agent_ids_res, num_active_agents, num_pending_agents) =
            get_agent_ids(&app, &contract_addr);

        let mut error_msg = check_in_res.unwrap_err();
        assert_eq!(
            ContractError::CustomError {
                val: "Not accepting new agents".to_string()
            },
            error_msg.downcast().unwrap()
        );

        // Again, add two more tasks so we can nominate another agent
        add_task_exec(&mut app, &contract_addr, PARTICIPANT3);
        let (agent_ids_res, num_active_agents, num_pending_agents) =
            get_agent_ids(&app, &contract_addr);
        add_task_exec(&mut app, &contract_addr, PARTICIPANT4);

        // Add another agent, since there's now the need
        register_agent_exec(&mut app, &contract_addr, AGENT4, &AGENT_BENEFICIARY);
        // Fast forward time past the duration of the first pending agent,
        // allowing the second to nominate themselves
        app.update_block(add_one_duration_of_time);

        // debugging
        let (agent_ids_res, num_active_agents, num_pending_agents) =
            get_agent_ids(&app, &contract_addr);

        // Agent second in line nominates themself
        check_in_res = check_in_exec(&mut app, &contract_addr, AGENT4);
        assert!(
            check_in_res.is_ok(),
            "Agent second in line should be able to nominate themselves"
        );

        let (agent_ids_res, num_active_agents, num_pending_agents) =
            get_agent_ids(&app, &contract_addr);

        // Ensure the pending list is empty, having the earlier index booted
        assert_eq!(
            num_pending_agents, 0,
            "Expect the pending queue to be empty"
        );
    }
}
