use cosm_orc::orchestrator::cosm_orc::CosmOrc;
use cosm_orc::orchestrator::error::ProcessError;
use cosm_orc::orchestrator::{CosmosgRPC, InstantiateResponse};
use stargaze_fair_burn::msg::InstantiateMsg as FairBurnInstantiateMsg;

use crate::helpers::chain::SigningAccount;
use crate::helpers::constants::NAME_FAIR_BURN;

pub fn instantiate_fair_burn(
    orc: &mut CosmOrc<CosmosgRPC>,
    user: &SigningAccount,
) -> Result<InstantiateResponse, ProcessError> {
    orc.instantiate(
        NAME_FAIR_BURN,
        &format!("{}_inst", NAME_FAIR_BURN,),
        &FairBurnInstantiateMsg { fee_bps: 5000 },
        &user.key,
        Some(user.account.address.parse().unwrap()),
        vec![],
    )
}
