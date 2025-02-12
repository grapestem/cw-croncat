/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.14.2.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { Addr, Uint128, Timestamp, Uint64, SlotType, AgentStatus, CosmosMsgForEmpty, BankMsg, StakingMsg, DistributionMsg, Binary, IbcMsg, WasmMsg, GovMsg, VoteOption, Boundary, Interval, Rule, Balance, NativeBalance, Status, ValueIndex, ValueOrdering, Croncat, Agent, GenericBalance, Cw20CoinVerified, Coin, GetBalancesResponse, GetConfigResponse, GetAgentIdsResponse, AgentResponse, AgentTaskResponse, GetSlotHashesResponse, GetSlotIdsResponse, TaskResponse, ActionForEmpty, Empty, IbcTimeout, IbcTimeoutBlock, HasBalanceGte, CheckOwnerOfNft, CheckProposalStatus, GenericQuery, GetWalletBalancesResponse, Task, BoundaryValidated, TaskRequest, Cw20Coin, ExecuteMsg, Cw20ReceiveMsg, GetAgentResponse, GetAgentTasksResponse, RoundRobinBalancerModeResponse, GetStateResponse, BalancesResponse, SlotResponse, SlotWithRuleResponse, ReplyQueueResponse, QueueItemResponse, TaskWithRulesResponse, GetTaskHashResponse, GetTaskResponse, GetTasksByOwnerResponse, GetTasksResponse, GetTasksWithRulesResponse, InstantiateMsg, QueryMsg, ValidateIntervalResponse } from "./CwCroncat.types";
export interface CwCroncatReadOnlyInterface {
  contractAddress: string;
  getConfig: () => Promise<GetConfigResponse>;
  getBalances: () => Promise<GetBalancesResponse>;
  getAgent: ({
    accountId
  }: {
    accountId: string;
  }) => Promise<GetAgentResponse>;
  getAgentIds: () => Promise<GetAgentIdsResponse>;
  getAgentTasks: ({
    accountId
  }: {
    accountId: string;
  }) => Promise<GetAgentTasksResponse>;
  getTasks: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<GetTasksResponse>;
  getTasksWithRules: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<GetTasksWithRulesResponse>;
  getTasksByOwner: ({
    ownerId
  }: {
    ownerId: string;
  }) => Promise<GetTasksByOwnerResponse>;
  getTask: ({
    taskHash
  }: {
    taskHash: string;
  }) => Promise<GetTaskResponse>;
  getTaskHash: ({
    task
  }: {
    task: Task;
  }) => Promise<GetTaskHashResponse>;
  validateInterval: ({
    interval
  }: {
    interval: Interval;
  }) => Promise<ValidateIntervalResponse>;
  getSlotHashes: ({
    slot
  }: {
    slot?: number;
  }) => Promise<GetSlotHashesResponse>;
  getSlotIds: () => Promise<GetSlotIdsResponse>;
  getWalletBalances: ({
    wallet
  }: {
    wallet: string;
  }) => Promise<GetWalletBalancesResponse>;
  getState: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<GetStateResponse>;
}
export class CwCroncatQueryClient implements CwCroncatReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.getConfig = this.getConfig.bind(this);
    this.getBalances = this.getBalances.bind(this);
    this.getAgent = this.getAgent.bind(this);
    this.getAgentIds = this.getAgentIds.bind(this);
    this.getAgentTasks = this.getAgentTasks.bind(this);
    this.getTasks = this.getTasks.bind(this);
    this.getTasksWithRules = this.getTasksWithRules.bind(this);
    this.getTasksByOwner = this.getTasksByOwner.bind(this);
    this.getTask = this.getTask.bind(this);
    this.getTaskHash = this.getTaskHash.bind(this);
    this.validateInterval = this.validateInterval.bind(this);
    this.getSlotHashes = this.getSlotHashes.bind(this);
    this.getSlotIds = this.getSlotIds.bind(this);
    this.getWalletBalances = this.getWalletBalances.bind(this);
    this.getState = this.getState.bind(this);
  }

  getConfig = async (): Promise<GetConfigResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_config: {}
    });
  };
  getBalances = async (): Promise<GetBalancesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_balances: {}
    });
  };
  getAgent = async ({
    accountId
  }: {
    accountId: string;
  }): Promise<GetAgentResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_agent: {
        account_id: accountId
      }
    });
  };
  getAgentIds = async (): Promise<GetAgentIdsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_agent_ids: {}
    });
  };
  getAgentTasks = async ({
    accountId
  }: {
    accountId: string;
  }): Promise<GetAgentTasksResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_agent_tasks: {
        account_id: accountId
      }
    });
  };
  getTasks = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<GetTasksResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_tasks: {
        from_index: fromIndex,
        limit
      }
    });
  };
  getTasksWithRules = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<GetTasksWithRulesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_tasks_with_rules: {
        from_index: fromIndex,
        limit
      }
    });
  };
  getTasksByOwner = async ({
    ownerId
  }: {
    ownerId: string;
  }): Promise<GetTasksByOwnerResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_tasks_by_owner: {
        owner_id: ownerId
      }
    });
  };
  getTask = async ({
    taskHash
  }: {
    taskHash: string;
  }): Promise<GetTaskResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_task: {
        task_hash: taskHash
      }
    });
  };
  getTaskHash = async ({
    task
  }: {
    task: Task;
  }): Promise<GetTaskHashResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_task_hash: {
        task
      }
    });
  };
  validateInterval = async ({
    interval
  }: {
    interval: Interval;
  }): Promise<ValidateIntervalResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      validate_interval: {
        interval
      }
    });
  };
  getSlotHashes = async ({
    slot
  }: {
    slot?: number;
  }): Promise<GetSlotHashesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_slot_hashes: {
        slot
      }
    });
  };
  getSlotIds = async (): Promise<GetSlotIdsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_slot_ids: {}
    });
  };
  getWalletBalances = async ({
    wallet
  }: {
    wallet: string;
  }): Promise<GetWalletBalancesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_wallet_balances: {
        wallet
      }
    });
  };
  getState = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<GetStateResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_state: {
        from_index: fromIndex,
        limit
      }
    });
  };
}
export interface CwCroncatInterface extends CwCroncatReadOnlyInterface {
  contractAddress: string;
  sender: string;
  updateSettings: ({
    agentFee,
    agentsEjectThreshold,
    gasPrice,
    minTasksPerAgent,
    ownerId,
    paused,
    proxyCallbackGas,
    slotGranularity
  }: {
    agentFee?: Coin;
    agentsEjectThreshold?: number;
    gasPrice?: number;
    minTasksPerAgent?: number;
    ownerId?: string;
    paused?: boolean;
    proxyCallbackGas?: number;
    slotGranularity?: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  moveBalances: ({
    accountId,
    balances
  }: {
    accountId: string;
    balances: Balance[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  registerAgent: ({
    payableAccountId
  }: {
    payableAccountId?: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  updateAgent: ({
    payableAccountId
  }: {
    payableAccountId: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  checkInAgent: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  unregisterAgent: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  withdrawReward: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  createTask: ({
    task
  }: {
    task: TaskRequest;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  removeTask: ({
    taskHash
  }: {
    taskHash: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  refillTaskBalance: ({
    taskHash
  }: {
    taskHash: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  refillTaskCw20Balance: ({
    cw20Coins,
    taskHash
  }: {
    cw20Coins: Cw20Coin[];
    taskHash: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  proxyCall: ({
    taskHash
  }: {
    taskHash?: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  receive: ({
    amount,
    msg,
    sender
  }: {
    amount: Uint128;
    msg: Binary;
    sender: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  withdrawWalletBalance: ({
    cw20Amounts
  }: {
    cw20Amounts: Cw20Coin[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class CwCroncatClient extends CwCroncatQueryClient implements CwCroncatInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.updateSettings = this.updateSettings.bind(this);
    this.moveBalances = this.moveBalances.bind(this);
    this.registerAgent = this.registerAgent.bind(this);
    this.updateAgent = this.updateAgent.bind(this);
    this.checkInAgent = this.checkInAgent.bind(this);
    this.unregisterAgent = this.unregisterAgent.bind(this);
    this.withdrawReward = this.withdrawReward.bind(this);
    this.createTask = this.createTask.bind(this);
    this.removeTask = this.removeTask.bind(this);
    this.refillTaskBalance = this.refillTaskBalance.bind(this);
    this.refillTaskCw20Balance = this.refillTaskCw20Balance.bind(this);
    this.proxyCall = this.proxyCall.bind(this);
    this.receive = this.receive.bind(this);
    this.withdrawWalletBalance = this.withdrawWalletBalance.bind(this);
  }

  updateSettings = async ({
    agentFee,
    agentsEjectThreshold,
    gasPrice,
    minTasksPerAgent,
    ownerId,
    paused,
    proxyCallbackGas,
    slotGranularity
  }: {
    agentFee?: Coin;
    agentsEjectThreshold?: number;
    gasPrice?: number;
    minTasksPerAgent?: number;
    ownerId?: string;
    paused?: boolean;
    proxyCallbackGas?: number;
    slotGranularity?: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_settings: {
        agent_fee: agentFee,
        agents_eject_threshold: agentsEjectThreshold,
        gas_price: gasPrice,
        min_tasks_per_agent: minTasksPerAgent,
        owner_id: ownerId,
        paused,
        proxy_callback_gas: proxyCallbackGas,
        slot_granularity: slotGranularity
      }
    }, fee, memo, funds);
  };
  moveBalances = async ({
    accountId,
    balances
  }: {
    accountId: string;
    balances: Balance[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      move_balances: {
        account_id: accountId,
        balances
      }
    }, fee, memo, funds);
  };
  registerAgent = async ({
    payableAccountId
  }: {
    payableAccountId?: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      register_agent: {
        payable_account_id: payableAccountId
      }
    }, fee, memo, funds);
  };
  updateAgent = async ({
    payableAccountId
  }: {
    payableAccountId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_agent: {
        payable_account_id: payableAccountId
      }
    }, fee, memo, funds);
  };
  checkInAgent = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      check_in_agent: {}
    }, fee, memo, funds);
  };
  unregisterAgent = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      unregister_agent: {}
    }, fee, memo, funds);
  };
  withdrawReward = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_reward: {}
    }, fee, memo, funds);
  };
  createTask = async ({
    task
  }: {
    task: TaskRequest;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      create_task: {
        task
      }
    }, fee, memo, funds);
  };
  removeTask = async ({
    taskHash
  }: {
    taskHash: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove_task: {
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  refillTaskBalance = async ({
    taskHash
  }: {
    taskHash: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      refill_task_balance: {
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  refillTaskCw20Balance = async ({
    cw20Coins,
    taskHash
  }: {
    cw20Coins: Cw20Coin[];
    taskHash: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      refill_task_cw20_balance: {
        cw20_coins: cw20Coins,
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  proxyCall = async ({
    taskHash
  }: {
    taskHash?: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      proxy_call: {
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  receive = async ({
    amount,
    msg,
    sender
  }: {
    amount: Uint128;
    msg: Binary;
    sender: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      receive: {
        amount,
        msg,
        sender
      }
    }, fee, memo, funds);
  };
  withdrawWalletBalance = async ({
    cw20Amounts
  }: {
    cw20Amounts: Cw20Coin[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_wallet_balance: {
        cw20_amounts: cw20Amounts
      }
    }, fee, memo, funds);
  };
}