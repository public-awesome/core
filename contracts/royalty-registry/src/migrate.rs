use crate::{
    error::ContractError,
    instantiate::{CONTRACT_NAME, CONTRACT_VERSION},
};

use cosmwasm_std::{DepsMut, Empty, Env};
use cw2::set_contract_version;
use sg_std::Response;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}
