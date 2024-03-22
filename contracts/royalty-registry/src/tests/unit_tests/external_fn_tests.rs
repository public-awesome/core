use crate::{
    msg::{QueryMsg, RoyaltyPaymentResponse},
    tests::setup::{
        setup_contracts::setup_royalty_registry,
        setup_dummy_contract::{setup_dummy_contract, TestExecuteMsg},
        setup_minter::standard_minter_template,
    },
};

use cosmwasm_std::Addr;
use cw_multi_test::Executor;

#[test]
fn try_fetch_or_set_royalties() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let royalty_registry = setup_royalty_registry(&mut router, creator.clone());
    let dummy_contract = setup_dummy_contract(&mut router, creator);
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();
    let _protocol = Addr::unchecked("protocol");

    // Assert there is no default royalty entry for a collection to start
    let royalty_payment_response: RoyaltyPaymentResponse = router
        .wrap()
        .query_wasm_smart(
            royalty_registry.clone(),
            &QueryMsg::RoyaltyPayment {
                collection: collection.to_string(),
                protocol: None,
            },
        )
        .unwrap();
    assert!(royalty_payment_response.royalty_default.is_none());
    assert!(royalty_payment_response.royalty_protocol.is_none());

    // Invoke fetch_or_set_royalties with no protocol address, should set default royalties
    let msg = TestExecuteMsg::TestFetchOrSetRoyalties {
        royalty_registry: royalty_registry.to_string(),
        collection: collection.to_string(),
        protocol: None,
    };
    let response = router.execute_contract(bidder, dummy_contract, &msg, &[]);
    assert!(response.is_ok());

    let royalty_payment_response: RoyaltyPaymentResponse = router
        .wrap()
        .query_wasm_smart(
            royalty_registry,
            &QueryMsg::RoyaltyPayment {
                collection: collection.to_string(),
                protocol: None,
            },
        )
        .unwrap();
    assert!(royalty_payment_response.royalty_default.is_some());
    assert!(royalty_payment_response.royalty_protocol.is_none());
}
