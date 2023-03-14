#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
  Binary,
  Deps,
  DepsMut,
  Env,
  MessageInfo,
  Response,
  StdResult,
  to_binary,
};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::execute;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg };
use crate::state::{ ContractState, CONTRACT_STATE };

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-template";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: InstantiateMsg
) -> Result<Response, ContractError> {
  let state = ContractState {
    period_finish_at: msg.period_finish_at,
    reward_rate: msg.reward_rate,
    last_update_at: 0,
    total_supply: 0,
    reward_per_token_stored: 0,
    staking_token: msg.staking_token,
    reward_token: msg.reward_token,
  };

  CONTRACT_STATE.save(deps.storage, &state)?;
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

  Ok(Response::new().add_attribute("method", "instantiate").add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::StakeMsg { amount } => execute::stake(deps, env, info, amount),
    // ExecuteMsg::Withdraw {amount} => execute::wi(deps, amount),
    _ => Err(ContractError::Unauthorized {}),
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
    QueryMsg::GetStaked { account } => to_binary(&query::get_staked(deps, account).unwrap()),
    // _ => Err(StdError::generic_err("unimplemented")),
  }
}

pub mod query {
  use cosmwasm_std::{ Deps, Addr };

  use crate::{ state::{ USERS }, msg::GetStakedResponse, ContractError };

  pub fn get_staked(deps: Deps, account: Addr) -> Result<GetStakedResponse, ContractError> {
    let state = USERS.load(deps.storage, &account)?;
    Ok(GetStakedResponse { balance: state.balance, reward: state.reward })
  }
}
