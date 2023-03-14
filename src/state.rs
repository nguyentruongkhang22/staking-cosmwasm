use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ Addr };
use cw_storage_plus::{ Item, Map };

#[cw_serde]
pub struct UserState {
  pub owner: Addr,
  pub balance: u64,
  pub reward: u64,
  pub reward_per_token_paid: u64,
}

#[cw_serde]
pub struct ContractState {
  pub staking_token: Addr,
  pub reward_token: Addr,
  pub period_finish_at: u64,
  pub reward_rate: u64,
  pub last_update_at: u64,
  pub total_supply: u64,
  pub reward_per_token_stored: u64,
}

pub const USERS: Map<&Addr, UserState> = Map::new("users");
pub const CONTRACT_STATE: Item<ContractState> = Item::new("contract-state");
