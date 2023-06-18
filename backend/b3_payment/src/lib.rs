use b3_helper_lib::time::NanoTimeStamp;
use b3_helper_lib::types::{CanisterId, RequestId};
use b3_permit_lib::processed::processed::ProcessedRequest;
use b3_permit_lib::request::request::Request;
use b3_permit_lib::types::{PendingRequestList, ProcessedRequestList};
use ic_cdk::export::candid::candid_method;
use ic_cdk::update;

// UPDATE ---------------------------------------------------------------------
#[candid_method(update)]
#[update]
pub async fn request_maker(
    canister_id: CanisterId,
    request: Request,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let b3_wallet = WalletCanister(canister_id);

    let request_id = b3_wallet
        .request_maker(request, reason, deadline)
        .await
        .unwrap();

    request_id
}

#[candid_method(update)]
#[update]
pub async fn request_connect(canister_id: CanisterId) -> RequestId {
    let b3_wallet = WalletCanister(canister_id);

    let request_id = b3_wallet.request_connect().await.unwrap();

    request_id
}

#[candid_method(update)]
#[update]
pub async fn is_connected(canister_id: CanisterId) -> bool {
    let b3_wallet = WalletCanister(canister_id);

    b3_wallet.validate_signer().await.unwrap()
}

#[candid_method(update)]
#[update]
pub async fn check_processed_request(
    canister_id: CanisterId,
    request_id: RequestId,
) -> ProcessedRequest {
    let b3_wallet = WalletCanister(canister_id);

    let processed_request = b3_wallet.get_processed(request_id).await.unwrap();

    processed_request
}

#[candid_method(update)]
#[update]
pub async fn check_pending_requests(canister_id: CanisterId) -> PendingRequestList {
    let b3_wallet = WalletCanister(canister_id);

    let processed_request = b3_wallet.get_pending_list().await.unwrap();

    processed_request
}

#[candid_method(update)]
#[update]
pub async fn check_processed_requests(canister_id: CanisterId) -> ProcessedRequestList {
    let b3_wallet = WalletCanister(canister_id);

    let processed_request = b3_wallet.get_processed_list().await.unwrap();

    processed_request
}

pub struct WalletCanister(pub CanisterId);

impl WalletCanister {
    pub async fn request_maker(
        &self,
        request: Request,
        reason: String,
        deadline: Option<NanoTimeStamp>,
    ) -> Result<RequestId, String> {
        let canister_id = self.0;

        let (request_id,): (RequestId,) =
            ic_cdk::call(canister_id, "request_maker", (request, reason, deadline))
                .await
                .map_err(|err| err.1)?;

        Ok(request_id)
    }

    pub async fn validate_signer(&self) -> Result<bool, String> {
        let canister_id = self.0;

        let (is_valid,): (bool,) = ic_cdk::call(canister_id, "validate_signer", (ic_cdk::id(),))
            .await
            .map_err(|err| err.1)?;

        Ok(is_valid)
    }

    pub async fn get_processed(&self, request_id: RequestId) -> Result<ProcessedRequest, String> {
        let canister_id = self.0;

        let (processed_request,): (ProcessedRequest,) =
            ic_cdk::call(canister_id, "get_processed", (request_id,))
                .await
                .map_err(|err| err.1)?;

        Ok(processed_request)
    }

    pub async fn request_connect(&self) -> Result<RequestId, String> {
        let canister_id = self.0;

        let (request_id,): (RequestId,) = ic_cdk::call(canister_id, "request_connect", ())
            .await
            .map_err(|err| err.1)?;

        Ok(request_id)
    }

    pub async fn get_pending_list(&self) -> Result<PendingRequestList, String> {
        let canister_id = self.0;

        let (pending_request_list,): (PendingRequestList,) =
            ic_cdk::call(canister_id, "get_pending_list", ())
                .await
                .map_err(|err| err.1)?;

        Ok(pending_request_list)
    }

    pub async fn get_processed_list(&self) -> Result<ProcessedRequestList, String> {
        let canister_id = self.0;

        let (processed_request_list,): (ProcessedRequestList,) =
            ic_cdk::call(canister_id, "get_processed_list", ())
                .await
                .map_err(|err| err.1)?;

        Ok(processed_request_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::export_service;

    #[test]
    fn generate_candid() {
        use std::io::Write;

        let mut file = std::fs::File::create("./b3_payment.did").unwrap();

        export_service!();

        let candid = __export_service();

        file.write_all(candid.as_bytes()).unwrap();

        assert!(true);
    }
}
