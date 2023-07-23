use crate::{
    permit::{caller_is_admin, caller_is_signer},
    wallet::version,
};
use b3_operations::{
    operation::{
        btc::transfer::BtcTransfer,
        global::SendToken,
        icp::transfer::IcpTransfer,
        inner::{
            account::{CreateAccount, RemoveAccount, RenameAccount},
            setting::{UpdateCanisterSettings, UpgradeCanister},
            signer::AddSigner,
        },
        {OperationTrait, Operations},
    },
    pending::RequestArgs,
    signer::roles::SignerRoles,
    store::{with_permit, with_permit_mut, with_signer_check, with_signer_ids_by_role},
    types::PendingRequestList,
};
use b3_utils::{revert, timestamp::NanoTimeStamp, types::RequestId, wasm::with_wasm};
use candid::candid_method;
use ic_cdk::{query, update};

// QUERY ---------------------------------------------------------------------

#[candid_method(query)]
#[query(guard = "caller_is_signer")]
pub fn get_pending_list() -> PendingRequestList {
    with_permit(|s| s.pending_list())
}

#[candid_method(query)]
#[query(guard = "caller_is_signer")]
pub fn is_connected() -> bool {
    let caller = ic_cdk::caller();

    with_signer_check(caller, |signer| signer.is_canister()).is_ok()
}

// UPDATE ---------------------------------------------------------------------
#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn request_maker(
    request: Operations,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = SignerRoles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_add_signer(
    request: AddSigner,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = SignerRoles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[update]
#[candid_method(update)]
pub fn request_connect() -> RequestId {
    let caller = ic_cdk::caller();

    let request = AddSigner {
        name: "B3Peyment".to_string(),
        role: SignerRoles::Canister,
        signer_id: caller,
        expires_at: None,
        threshold: None,
    };

    with_permit(|s| {
        // check if the request is already in the pending list
        let pending_list = s.pending_list();

        for pending_request in pending_list.iter() {
            if pending_request.request == Operations::AddSigner(request.clone()) {
                return revert("Already Pending!");
            }
        }
    });

    if with_signer_check(caller, |signer| signer.is_canister()).is_ok() {
        return revert("Already connected!");
    }

    let role = SignerRoles::Admin;
    let allowed_signers = with_signer_ids_by_role(role.clone(), |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason: format!("Connecting to B3Payment for making payment!"),
        deadline: None,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_update_settings(
    request: UpdateCanisterSettings,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    request.validate_request().unwrap_or_else(revert);

    let role = SignerRoles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_account_rename(
    request: RenameAccount,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = SignerRoles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_create_account(
    request: CreateAccount,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = SignerRoles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_delete_account(
    request: RemoveAccount,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = SignerRoles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_transfer_icp(
    request: IcpTransfer,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = SignerRoles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_transfer_btc(
    request: BtcTransfer,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = SignerRoles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn request_send(
    request: SendToken,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    request.validate_request().unwrap_or_else(revert);

    let role = SignerRoles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);

        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn request_upgrade_canister(wasm_version: String) -> RequestId {
    let caller = ic_cdk::caller();

    let upgrade_request = with_wasm(|w| UpgradeCanister {
        wasm_hash_string: w.generate_hash_string(),
        wasm_version,
    });

    upgrade_request.validate_request().unwrap_or_else(revert);

    let role = SignerRoles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        role,
        allowed_signers,
        request: upgrade_request.into(),
        version: version(),
        reason: "Upgrade canister".to_string(),
        deadline: None,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);

        s.insert_new_request(new_request)
    })
}
