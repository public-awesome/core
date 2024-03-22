use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::Config,
    state::{RoyaltyDefault, RoyaltyEntry, RoyaltyProtocol},
    tests::helpers::utils::assert_error,
    tests::setup::{
        setup_accounts::setup_accounts,
        setup_contracts::{contract_royalty_registry, setup_royalty_registry},
        setup_minter::standard_minter_template,
    },
    ContractError,
};

use cosmwasm_std::{Addr, Decimal};
use cw_multi_test::Executor;
use sg_std::GENESIS_MINT_START_TIME;
use test_suite::common_setup::{
    contract_boxes::custom_mock_app, setup_accounts_and_block::setup_block_time,
};

#[test]
fn try_instantiate() {
    let mut app = custom_mock_app();
    let royalty_registry_id = app.store_code(contract_royalty_registry());
    let (_owner, _bidder, creator) = setup_accounts(&mut app).unwrap();

    let update_wait_period = 6;
    let max_share_delta = Decimal::percent(1);

    let msg = InstantiateMsg {
        config: Config {
            update_wait_period,
            max_share_delta,
        },
    };

    let royalty_registry = app
        .instantiate_contract(royalty_registry_id, creator, &msg, &[], "auction", None)
        .unwrap();

    let config: Config = app
        .wrap()
        .query_wasm_smart(royalty_registry, &QueryMsg::Config {})
        .unwrap();

    assert_eq!(config.update_wait_period, update_wait_period);
    assert_eq!(config.max_share_delta, max_share_delta);
}

#[test]
fn try_initialize_collection_royalty() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let royalty_registry = setup_royalty_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let _block_time = router.block_info().time;

    let royalty_entry: Option<RoyaltyEntry> = router
        .wrap()
        .query_wasm_smart(
            royalty_registry.clone(),
            &QueryMsg::CollectionRoyaltyDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert!(royalty_entry.is_none());

    // Anyone can initialize a collection royalty default
    let msg = ExecuteMsg::InitializeCollectionRoyalty {
        collection: collection.to_string(),
    };

    let response = router.execute_contract(bidder.clone(), royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let royalty_default: Option<RoyaltyDefault> = router
        .wrap()
        .query_wasm_smart(
            royalty_registry.clone(),
            &QueryMsg::CollectionRoyaltyDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        royalty_default,
        Some(RoyaltyDefault {
            collection,
            royalty_entry: RoyaltyEntry {
                recipient: Addr::unchecked(creator),
                share: Decimal::percent(10),
                updated: None
            }
        })
    );

    // Initialize cannot be invoked twice
    let response = router.execute_contract(bidder, royalty_registry, &msg, &[]);
    assert_error(
        response,
        ContractError::InvalidCollectionRoyalty(
            "Collection royalty already initialized".to_string(),
        )
        .to_string(),
    );
}

#[test]
fn try_set_collection_royalty_default() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let royalty_registry = setup_royalty_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let block_time = router.block_info().time;

    let royalty_entry: Option<RoyaltyEntry> = router
        .wrap()
        .query_wasm_smart(
            royalty_registry.clone(),
            &QueryMsg::CollectionRoyaltyDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert!(royalty_entry.is_none());

    // Non collection owner cannot set collection royalty default
    let msg = ExecuteMsg::SetCollectionRoyaltyDefault {
        collection: collection.to_string(),
        recipient: bidder.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(bidder, royalty_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::Unauthorized("Only collection owner can execute this action".to_string())
            .to_string(),
    );

    // Collection owner cannot set collection royalty default above 100%
    let msg = ExecuteMsg::SetCollectionRoyaltyDefault {
        collection: collection.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(101),
    };
    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::InvalidCollectionRoyalty(
            "Royalty share must be less than or equal to 1".to_string(),
        )
        .to_string(),
    );

    // Collection owner can set collection royalty default
    let msg = ExecuteMsg::SetCollectionRoyaltyDefault {
        collection: collection.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let royalty_default: Option<RoyaltyDefault> = router
        .wrap()
        .query_wasm_smart(
            royalty_registry.clone(),
            &QueryMsg::CollectionRoyaltyDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        royalty_default,
        Some(RoyaltyDefault {
            collection: collection.clone(),
            royalty_entry: RoyaltyEntry {
                recipient: Addr::unchecked(creator.clone()),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        })
    );

    // Collection owner cannot set collection royalty default twice
    let msg = ExecuteMsg::SetCollectionRoyaltyDefault {
        collection: collection.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator, royalty_registry, &msg, &[]);
    assert_error(
        response,
        ContractError::InvalidCollectionRoyalty(
            "Collection royalty already initialized".to_string(),
        )
        .to_string(),
    );
}

#[test]
fn try_update_collection_royalty_default() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let royalty_registry = setup_royalty_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();

    let config: Config = router
        .wrap()
        .query_wasm_smart(royalty_registry.clone(), &QueryMsg::Config {})
        .unwrap();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let block_time = router.block_info().time;

    let msg = ExecuteMsg::SetCollectionRoyaltyDefault {
        collection: collection.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    // Non collection owner cannot update collection royalty default
    let msg = ExecuteMsg::UpdateCollectionRoyaltyDefault {
        collection: collection.to_string(),
        recipient: Some(bidder.to_string()),
        share_delta: None,
        decrement: None,
    };

    let response = router.execute_contract(bidder.clone(), royalty_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::Unauthorized("Only collection owner can execute this action".to_string())
            .to_string(),
    );

    // Collection owner cannot update collection royalty default within wait period
    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::Unauthorized("Royalty entry cannot be updated yet".to_string()).to_string(),
    );

    // Collection owner can update collection royalty default outside of wait period
    setup_block_time(
        &mut router,
        block_time.plus_seconds(config.update_wait_period).nanos(),
        None,
    );
    let block_time = router.block_info().time;
    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let royalty_default: Option<RoyaltyDefault> = router
        .wrap()
        .query_wasm_smart(
            royalty_registry.clone(),
            &QueryMsg::CollectionRoyaltyDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        royalty_default,
        Some(RoyaltyDefault {
            collection: collection.clone(),
            royalty_entry: RoyaltyEntry {
                recipient: Addr::unchecked(bidder.clone()),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        })
    );

    // Collection owner can increment collection royalty default shares, but not more than max_share_delta
    setup_block_time(
        &mut router,
        block_time.plus_seconds(config.update_wait_period).nanos(),
        None,
    );
    let block_time = router.block_info().time;
    let msg = ExecuteMsg::UpdateCollectionRoyaltyDefault {
        collection: collection.to_string(),
        recipient: None,
        share_delta: Some(Decimal::percent(10)),
        decrement: None,
    };
    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let royalty_default: Option<RoyaltyDefault> = router
        .wrap()
        .query_wasm_smart(
            royalty_registry.clone(),
            &QueryMsg::CollectionRoyaltyDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        royalty_default,
        Some(RoyaltyDefault {
            collection: collection.clone(),
            royalty_entry: RoyaltyEntry {
                recipient: Addr::unchecked(bidder.clone()),
                share: Decimal::percent(11),
                updated: Some(block_time)
            }
        })
    );

    // Collection owner can decrement collection royalty default shares, but not more than max_share_delta
    setup_block_time(
        &mut router,
        block_time.plus_seconds(config.update_wait_period).nanos(),
        None,
    );
    let block_time = router.block_info().time;
    let msg = ExecuteMsg::UpdateCollectionRoyaltyDefault {
        collection: collection.to_string(),
        recipient: None,
        share_delta: Some(Decimal::percent(10)),
        decrement: Some(true),
    };
    let response = router.execute_contract(creator, royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let royalty_default: Option<RoyaltyDefault> = router
        .wrap()
        .query_wasm_smart(
            royalty_registry,
            &QueryMsg::CollectionRoyaltyDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        royalty_default,
        Some(RoyaltyDefault {
            collection,
            royalty_entry: RoyaltyEntry {
                recipient: Addr::unchecked(bidder),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        })
    );
}

#[test]
fn try_set_collection_royalty_protocol() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let royalty_registry = setup_royalty_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();
    let protocol = Addr::unchecked("protocol");

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let block_time = router.block_info().time;

    let royalty_entry: Option<RoyaltyEntry> = router
        .wrap()
        .query_wasm_smart(
            royalty_registry.clone(),
            &QueryMsg::CollectionRoyaltyDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert!(royalty_entry.is_none());

    // Non collection owner cannot set collection royalty protocol
    let msg = ExecuteMsg::SetCollectionRoyaltyProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: bidder.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(bidder, royalty_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::Unauthorized("Only collection owner can execute this action".to_string())
            .to_string(),
    );

    // Collection owner can set collection royalty protocol
    let msg = ExecuteMsg::SetCollectionRoyaltyProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let royalty_protocol: Option<RoyaltyProtocol> = router
        .wrap()
        .query_wasm_smart(
            royalty_registry.clone(),
            &QueryMsg::CollectionRoyaltyProtocol {
                collection: collection.to_string(),
                protocol: protocol.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        royalty_protocol,
        Some(RoyaltyProtocol {
            collection: collection.clone(),
            protocol: protocol.clone(),
            royalty_entry: RoyaltyEntry {
                recipient: Addr::unchecked(creator.clone()),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        })
    );

    // Collection owner cannot set collection royalty default twice
    let msg = ExecuteMsg::SetCollectionRoyaltyProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::InvalidCollectionRoyalty(
            "Collection royalty protocol already initialized".to_string(),
        )
        .to_string(),
    );

    let royalty_protocols = router
        .wrap()
        .query_wasm_smart::<Vec<RoyaltyProtocol>>(
            royalty_registry,
            &QueryMsg::RoyaltyProtocolByCollection {
                collection: collection.to_string(),
                query_options: None,
            },
        )
        .unwrap();

    assert_eq!(royalty_protocols.len(), 1);
    assert_eq!(
        royalty_protocols[0],
        RoyaltyProtocol {
            collection,
            protocol,
            royalty_entry: RoyaltyEntry {
                recipient: Addr::unchecked(creator),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        }
    );
}

#[test]
fn try_update_collection_royalty_protocol() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let royalty_registry = setup_royalty_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();
    let protocol = Addr::unchecked("protocol");

    let config: Config = router
        .wrap()
        .query_wasm_smart(royalty_registry.clone(), &QueryMsg::Config {})
        .unwrap();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let block_time = router.block_info().time;

    let msg = ExecuteMsg::SetCollectionRoyaltyProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    // Non collection owner cannot update collection royalty default
    let msg = ExecuteMsg::UpdateCollectionRoyaltyProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: Some(bidder.to_string()),
        share_delta: None,
        decrement: None,
    };

    let response = router.execute_contract(bidder.clone(), royalty_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::Unauthorized("Only collection owner can execute this action".to_string())
            .to_string(),
    );

    // Collection owner cannot update collection royalty default within wait period
    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::Unauthorized("Royalty entry cannot be updated yet".to_string()).to_string(),
    );

    // Collection owner can update collection royalty default outside of wait period
    setup_block_time(
        &mut router,
        block_time.plus_seconds(config.update_wait_period).nanos(),
        None,
    );
    let block_time = router.block_info().time;
    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let royalty_protocol: Option<RoyaltyProtocol> = router
        .wrap()
        .query_wasm_smart(
            royalty_registry.clone(),
            &QueryMsg::CollectionRoyaltyProtocol {
                collection: collection.to_string(),
                protocol: protocol.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        royalty_protocol,
        Some(RoyaltyProtocol {
            collection: collection.clone(),
            protocol: protocol.clone(),
            royalty_entry: RoyaltyEntry {
                recipient: Addr::unchecked(bidder.clone()),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        })
    );

    // Collection owner can increment collection royalty default shares, but not more than max_share_delta
    setup_block_time(
        &mut router,
        block_time.plus_seconds(config.update_wait_period).nanos(),
        None,
    );
    let block_time = router.block_info().time;
    let msg = ExecuteMsg::UpdateCollectionRoyaltyProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: None,
        share_delta: Some(Decimal::percent(10)),
        decrement: None,
    };
    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let royalty_protocol: Option<RoyaltyProtocol> = router
        .wrap()
        .query_wasm_smart(
            royalty_registry.clone(),
            &QueryMsg::CollectionRoyaltyProtocol {
                collection: collection.to_string(),
                protocol: protocol.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        royalty_protocol,
        Some(RoyaltyProtocol {
            collection: collection.clone(),
            protocol: protocol.clone(),
            royalty_entry: RoyaltyEntry {
                recipient: Addr::unchecked(bidder.clone()),
                share: Decimal::percent(11),
                updated: Some(block_time)
            }
        })
    );

    // Collection owner can decrement collection royalty default shares, but not more than max_share_delta
    setup_block_time(
        &mut router,
        block_time.plus_seconds(config.update_wait_period).nanos(),
        None,
    );
    let block_time = router.block_info().time;
    let msg = ExecuteMsg::UpdateCollectionRoyaltyProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: None,
        share_delta: Some(Decimal::percent(10)),
        decrement: Some(true),
    };
    let response = router.execute_contract(creator, royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let royalty_protocol: Option<RoyaltyProtocol> = router
        .wrap()
        .query_wasm_smart(
            royalty_registry,
            &QueryMsg::CollectionRoyaltyProtocol {
                collection: collection.to_string(),
                protocol: protocol.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        royalty_protocol,
        Some(RoyaltyProtocol {
            collection,
            protocol,
            royalty_entry: RoyaltyEntry {
                recipient: Addr::unchecked(bidder),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        })
    );
}

#[test]
fn try_over_100_percent_royalty() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let royalty_registry = setup_royalty_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();
    let protocol = Addr::unchecked("protocol");

    let config: Config = router
        .wrap()
        .query_wasm_smart(royalty_registry.clone(), &QueryMsg::Config {})
        .unwrap();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let mut block_time = router.block_info().time;

    let msg = ExecuteMsg::SetCollectionRoyaltyProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    // Collection owner can not exceed 100% royalty. Test 101% royalty
    let msg = ExecuteMsg::UpdateCollectionRoyaltyProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: Some(bidder.to_string()),
        share_delta: Some(Decimal::percent(10)),
        decrement: None,
    };
    for i in 1..=91 {
        block_time = block_time.plus_seconds(config.update_wait_period);
        setup_block_time(&mut router, block_time.nanos(), None);
        let response =
            router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
        // 10 + 91 = 101% > 100% max royalty
        if i == 91 {
            assert_error(
                response,
                ContractError::InvalidCollectionRoyalty(
                    "Royalty share must be less than or equal to 1".to_string(),
                )
                .to_string(),
            );
        } else {
            assert!(response.is_ok());
        }
    }
}

#[test]
fn try_0_royalty() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let royalty_registry = setup_royalty_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();
    let protocol = Addr::unchecked("protocol");

    let config: Config = router
        .wrap()
        .query_wasm_smart(royalty_registry.clone(), &QueryMsg::Config {})
        .unwrap();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let mut block_time = router.block_info().time;

    let msg = ExecuteMsg::SetCollectionRoyaltyProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    // Collection owner can decrement to 0% royalty
    let msg = ExecuteMsg::UpdateCollectionRoyaltyProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: Some(bidder.to_string()),
        share_delta: Some(Decimal::percent(10)),
        decrement: Some(true),
    };
    for _ in 1..=10 {
        block_time = block_time.plus_seconds(config.update_wait_period);
        setup_block_time(&mut router, block_time.nanos(), None);

        let response =
            router.execute_contract(creator.clone(), royalty_registry.clone(), &msg, &[]);
        assert!(response.is_ok());
    }
}
