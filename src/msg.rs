use cosmwasm_schema::{ cw_serde, QueryResponses };
use cosmwasm_std::{Uint128, Addr};

#[cw_serde]
pub struct InstantiateMsg {
  pub staking_token: Addr,
  pub reward_token: Addr,
  pub period_finish_at: u64,
  pub reward_rate: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
  StakeMsg {
    amount: u64,
  },
  Withdraw {
    amount: u64,
  },
  TransferFrom {
    owner: String,
    recipient: String,
    amount: Uint128,
  },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
  // GetCount returns the current count as a json-encoded number
  #[returns(GetStakedResponse)] GetStaked {
    account: Addr,
  },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetStakedResponse {
  pub balance: u64,
  pub reward: u64,
}
