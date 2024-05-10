use cosmwasm_std::{Addr, Decimal, Empty};
use cw_multi_test::{Contract, ContractWrapper, Executor};
use test_suite::common_setup::contract_boxes::App;

pub fn contract_royalty_registry() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::execute::execute,
        crate::instantiate::instantiate,
        crate::query::query,
    )
    .with_sudo(crate::sudo::sudo);
    Box::new(contract)
}

pub fn setup_royalty_registry(router: &mut App, creator: Addr) -> Addr {
    let royalty_registry_id = router.store_code(contract_royalty_registry());
    let msg = crate::msg::InstantiateMsg {
        config: crate::state::Config {
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
