use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, CosmosMsg, QuerierWrapper, StdResult, WasmMsg};

use crate::{
    msg::{ExecuteMsg, QueryMsg},
    state::Config,
};

#[cw_serde]
pub struct MutableWhitelistContract(pub Addr);

impl MutableWhitelistContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    /// Execute --------

    pub fn add_address(&self, address: String) -> StdResult<CosmosMsg> {
        self.call(ExecuteMsg::AddAddress { address })
    }

    pub fn remove_address(&self, address: String) -> StdResult<CosmosMsg> {
        self.call(ExecuteMsg::RemoveAddress { address })
    }

    pub fn purge(&self) -> StdResult<CosmosMsg> {
        self.call(ExecuteMsg::Purge {})
    }

    /// Queries --------

    pub fn config(&self, querier: &QuerierWrapper) -> StdResult<Config> {
        let res: Config = querier.query_wasm_smart(self.addr(), &QueryMsg::Config {})?;
        Ok(res)
    }

    pub fn includes_address(&self, querier: &QuerierWrapper, address: String) -> StdResult<bool> {
        let res: bool =
            querier.query_wasm_smart(self.addr(), &QueryMsg::IncludesAddress { address })?;
        Ok(res)
    }

    pub fn count(&self, querier: &QuerierWrapper) -> StdResult<u64> {
        let res: u64 = querier.query_wasm_smart(self.addr(), &QueryMsg::Count {})?;
        Ok(res)
    }

    pub fn list(&self, querier: &QuerierWrapper) -> StdResult<Vec<String>> {
        let res: Vec<String> = querier.query_wasm_smart(self.addr(), &QueryMsg::List {})?;
        Ok(res)
    }
}
