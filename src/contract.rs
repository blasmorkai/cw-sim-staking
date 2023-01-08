#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use cosmwasm_std::{Coin, Uint128, StakingMsg, DistributionMsg};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-sim-staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;


    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer{ } => execute::transfer(deps, info),
        ExecuteMsg::Bond { val_addr } => execute::bond(deps, info, val_addr),
        ExecuteMsg::Unbond { val_addr } => execute::unbond(deps, info, val_addr),
        ExecuteMsg::Collect { val_addr } => execute::claim_rewards(deps, info, val_addr),
    }
}

pub mod execute {
    use super::*;

    pub fn transfer (_deps: DepsMut, _info: MessageInfo) -> Result<Response, ContractError> {
        // Receives funds via info.funds
        Ok(Response::new()
        .add_attribute("action", "transfer_received"))
    }


    pub fn bond (_deps: DepsMut, _info: MessageInfo, val_addr: String) -> Result<Response, ContractError> {

        let amount_coin = Coin{ denom: "ujunox".to_string(), amount: Uint128::from(100u128) };
        Ok(Response::new()
        .add_message(StakingMsg::Delegate {
            validator: val_addr,
            amount: amount_coin,
        })
        .add_attribute("action", "bond"))
    }

    pub fn unbond (_deps: DepsMut, _info: MessageInfo, val_addr: String) -> Result<Response, ContractError> {
        let amount_coin = Coin{ denom: "ujunox".to_string(), amount: Uint128::from(100u128) };
        Ok(Response::new()
        .add_message(StakingMsg::Undelegate {
            validator: val_addr,
            amount: amount_coin,
        })
        .add_attribute("action", "unbond"))
    }

    pub fn claim_rewards (_deps: DepsMut, _info: MessageInfo, val_addr: String) -> Result<Response, ContractError> {
        Ok(Response::new()
        .add_message(DistributionMsg::WithdrawDelegatorReward { validator: val_addr })
        .add_attribute("action", "collect_rewards"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance {} => to_binary(&query::balance(deps)?),
    }
}

pub mod query {
    use cosmwasm_std::Uint128;

    use super::*;

    pub fn balance(_deps: Deps) -> StdResult<Uint128> {
        Ok(Uint128::zero())
    }
}

#[cfg(test)]
mod tests {


 
}
