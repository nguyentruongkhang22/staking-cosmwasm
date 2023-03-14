use std::{ cmp::min };
use cosmwasm_std::*;
use crate::{ ContractError, state::{ CONTRACT_STATE, USERS, UserState }, msg::ExecuteMsg };

pub fn stake(
  mut deps: DepsMut,
  env: Env,
  info: MessageInfo,
  amount: u64
) -> Result<Response, ContractError> {
  if !USERS.has(deps.storage, &info.sender) {
    create_new_user(deps.branch(), info.clone(), amount)?;
  }

  update_reward(deps.branch(), env.clone(), info.sender.clone())?;

  let mut contract_state = CONTRACT_STATE.load(deps.storage).unwrap();
  contract_state.total_supply += amount;
  CONTRACT_STATE.save(deps.branch().storage, &contract_state)?;

  _transfer_non_native_token(
    &info.sender,
    &env.contract.address,
    &contract_state.staking_token,
    amount
  )?;

  Ok(Response::new().add_attribute("method", "stake"))
}

pub fn withdraw(
  mut deps: DepsMut,
  env: Env,
  info: MessageInfo,
  amount: u64
) -> Result<Response, ContractError> {
  update_reward(deps.branch(), env.clone(), info.clone().sender)?;
  let mut contract_state = CONTRACT_STATE.load(deps.storage)?;
  let mut user_state = USERS.load(deps.storage, &info.sender)?;

  ensure!(user_state.balance >= amount, ContractError::InsufficientBalance {});

  contract_state.total_supply -= amount;
  user_state.balance -= amount;
  CONTRACT_STATE.save(deps.branch().storage, &contract_state)?;
  USERS.save(deps.storage, &info.clone().sender, &user_state)?;

  _transfer_non_native_token(
    &env.contract.address,
    &info.sender,
    &contract_state.staking_token,
    amount
  )?;

  Ok(Response::new().add_attribute("method", "withdraw"))
}

pub fn claim_reward(
  mut deps: DepsMut,
  env: Env,
  info: MessageInfo
) -> Result<Response, ContractError> {
  update_reward(deps.branch(), env.clone(), info.clone().sender)?;
  let mut user_state = USERS.load(deps.storage, &info.sender)?;
  let contract_state = CONTRACT_STATE.load(deps.storage)?;

  _transfer_non_native_token(
    &env.clone().contract.address,
    &info.sender,
    &contract_state.reward_token,
    user_state.reward
  )?;

  user_state.reward = 0;
  USERS.save(deps.storage, &info.sender, &user_state)?;

  Ok(Response::new().add_attribute("method", "claim_reward"))
}

fn update_reward(deps: DepsMut, env: Env, account: Addr) -> Result<Response, ContractError> {
  let mut state = CONTRACT_STATE.load(deps.storage).unwrap();
  state.reward_per_token_stored = get_reward_per_token_stored(deps.as_ref(), env.clone());
  state.last_update_at = min(env.block.time.seconds(), state.period_finish_at);
  CONTRACT_STATE.save(deps.storage, &state).unwrap();

  let mut user = USERS.load(deps.storage, &account).unwrap();
  let reward_per_token_stored = get_reward_per_token_stored(deps.as_ref(), env);
  user.reward =
    (user.balance * (reward_per_token_stored - user.reward_per_token_paid)) / (1e-18 as u64);
  user.reward_per_token_paid = reward_per_token_stored;
  USERS.save(deps.storage, &account, &user).unwrap();
  Ok(Response::default())
}

fn _transfer_native_token(
  to: &Addr,
  token: &Addr,
  amount: u64,
  res: &mut Response
) -> Result<Response, ContractError> {
  res.messages.push(
    SubMsg::new(BankMsg::Send {
      to_address: to.to_string(),
      amount: coins(amount as u128, token),
    })
  );
  Ok(Response::new())
}

fn _transfer_non_native_token(
  from: &Addr,
  to: &Addr,
  token: &Addr,
  amount: u64
) -> Result<Response, ContractError> {
  let mut res: Response = Response::new();
  let msg = ExecuteMsg::TransferFrom {
    owner: from.clone().into_string(),
    recipient: to.clone().into_string(),
    amount: Uint128::from(amount),
  };

  let sub_msg = SubMsg::new(WasmMsg::Execute {
    contract_addr: token.to_string(),
    msg: to_binary(&msg)?,
    funds: vec![],
  });

  res.messages.push(sub_msg);
  Ok(res)
}

fn get_reward_per_token_stored(deps: Deps, env: Env) -> u64 {
  let contract_state = CONTRACT_STATE.load(deps.storage).unwrap();
  let total_supply = contract_state.total_supply;
  let reward_per_token_stored = contract_state.reward_per_token_stored;

  if total_supply == 0 {
    return reward_per_token_stored;
  }

  reward_per_token_stored +
    (env.block.time.minus_seconds(contract_state.last_update_at).seconds() *
      contract_state.reward_rate) /
      total_supply
}

fn create_new_user(deps: DepsMut, info: MessageInfo, amount: u64) -> Result<(), StdError> {
  USERS.save(
    deps.storage,
    &info.sender,
    &(UserState {
      balance: amount,
      owner: info.sender.clone(),
      reward: 0,
      reward_per_token_paid: 0,
    })
  )
}
