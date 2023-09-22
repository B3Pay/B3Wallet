use ic_cdk::api::management_canister::{
    main::{canister_status, CanisterStatusResponse},
    provisional::{CanisterId, CanisterIdRecord},
};
use std::fmt;

use crate::error::HelperError;

pub async fn ic_canister_status(
    canister_id: CanisterId,
) -> Result<CanisterStatusResponse, HelperError> {
    let (status,) = canister_status(CanisterIdRecord { canister_id })
        .await
        .map_err(|e| HelperError::CanisterStatusError(e.1))?;

    Ok(status)
}

pub fn revert<T, E: fmt::Display>(err: E) -> T {
    ic_cdk::trap(&format!("{}", err));
}
