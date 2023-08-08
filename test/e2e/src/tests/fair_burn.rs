use cosm_orc::orchestrator::{Coin as OrcCoin, Denom as OrcDenom, ExecReq};
use stargaze_fair_burn::msg::ExecuteMsg as FairBurnExecuteMsg;
use std::str::FromStr;
use test_context::test_context;

use crate::helpers::{chain::Chain, constants::NAME_FAIR_BURN};

#[test_context(Chain)]
#[test]
#[ignore]
fn test_fair_burn(chain: &mut Chain) {
    let denom = chain.cfg.orc_cfg.chain_cfg.denom.clone();
    let _prefix = chain.cfg.orc_cfg.chain_cfg.prefix.clone();

    let master_account = chain.cfg.users[1].clone();

    // Invoke the fair burn contract
    let execute_msg = FairBurnExecuteMsg::FairBurn { recipient: None };
    let reqs = vec![ExecReq {
        contract_name: NAME_FAIR_BURN.to_string(),
        msg: Box::new(execute_msg),
        funds: vec![OrcCoin {
            denom: OrcDenom::from_str(&denom).unwrap(),
            amount: 50u128,
        }],
    }];

    let exec_response = chain
        .orc
        .execute_batch("fair-burn", reqs, &master_account.key);

    println!("exec_response: {exec_response:?}");
    assert!(exec_response.is_ok());
}
