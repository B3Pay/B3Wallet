use b3_helper::types::SignerId;
use b3_wallet_lib::{
    error::WalletError,
    request::sign::SignRequest,
    store::{with_signer, with_state_mut},
    types::RequestId,
};

pub fn sign_request(
    from: SignerId,
    request: SignRequest,
    deadline: Option<u64>,
) -> Result<RequestId, WalletError> {
    let role = with_signer(from.clone(), |s| s.role)?;

    let new_request = with_state_mut(|s| s.new_request(role, request, deadline));

    let request_id = with_state_mut(|s| s.insert_request(new_request));

    Ok(request_id)
}
