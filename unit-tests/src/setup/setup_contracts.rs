use cosmwasm_std::{Addr, Decimal};
use cw_multi_test::{Contract, ContractWrapper, Executor};
use sg_multi_test::StargazeApp;
use sg_std::StargazeMsgWrapper;

pub fn contract_royalty_registry() -> Box<dyn Contract<StargazeMsgWrapper>> {
    let contract = ContractWrapper::new(
        stargaze_royalty_registry::execute::execute,
        stargaze_royalty_registry::instantiate::instantiate,
        stargaze_royalty_registry::query::query,
    )
    .with_sudo(stargaze_royalty_registry::sudo::sudo);
    Box::new(contract)
}

pub fn setup_royalty_registry(router: &mut StargazeApp, creator: Addr) -> Addr {
    let royalty_registry_id = router.store_code(contract_royalty_registry());
    let msg = stargaze_royalty_registry::msg::InstantiateMsg {
        config: stargaze_royalty_registry::state::Config {
            update_wait_period: 6,
            max_share_delta: Decimal::percent(1),
        },
    };
    router
        .instantiate_contract(
            royalty_registry_id,
            creator,
            &msg,
            &[],
            "stargaze_royalty_registry",
            None,
        )
        .unwrap()
}
