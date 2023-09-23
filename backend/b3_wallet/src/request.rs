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
            user::AddUser,
        },
        {Operation, OperationTrait},
    },
    pending::RequestArgs,
    role::Role,
    store::{with_operation, with_operation_mut, with_users_can_operate, with_verified_user},
    types::PendingOperations,
};
use b3_utils::{revert, types::OperationId, wasm::with_wasm, NanoTimeStamp};
use ic_cdk::{query, update};

// QUERY ---------------------------------------------------------------------

#[query(guard = "caller_is_signer")]
pub fn get_pending_list() -> PendingOperations {
    with_operation(|s| s.pending_list())
}

#[query(guard = "caller_is_signer")]
pub fn is_connected() -> bool {
    let caller = ic_cdk::caller();

    with_verified_user(caller, |signer| signer.is_canister()).is_ok()
}

// UPDATE ---------------------------------------------------------------------
#[update(guard = "caller_is_signer")]
pub fn request_maker(
    request: Operation,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> OperationId {
    let caller = ic_cdk::caller();

    let role = Role::Admin;
    let allowed_signers = with_users_can_operate(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_operation_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[update(guard = "caller_is_admin")]
pub fn request_add_signer(
    request: AddUser,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> OperationId {
    let caller = ic_cdk::caller();

    let role = Role::Admin;
    let allowed_signers = with_users_can_operate(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_operation_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[update]
pub fn request_connect() -> OperationId {
    let caller = ic_cdk::caller();

    let request = AddUser {
        name: "B3Peyment".to_string(),
        role: Role::Canister,
        signer_id: caller,
        expires_at: None,
        threshold: None,
    };

    with_operation(|s| {
        // check if the request is already in the pending list
        let pending_list = s.pending_list();

        for pending_request in pending_list.iter() {
            if pending_request.request == Operation::AddSigner(request.clone()) {
                return revert("Already Pending!");
            }
        }
    });

    if with_verified_user(caller, |signer| signer.is_canister()).is_ok() {
        return revert("Already connected!");
    }

    let role = Role::Admin;
    let allowed_signers = with_users_can_operate(role.clone(), |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason: format!("Connecting to B3Payment for making payment!"),
        deadline: None,
    };

    with_operation_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[update(guard = "caller_is_admin")]
pub fn request_update_settings(
    request: UpdateCanisterSettings,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> OperationId {
    let caller = ic_cdk::caller();

    request.validate_request().unwrap_or_else(revert);

    let role = Role::Admin;
    let allowed_signers = with_users_can_operate(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_operation_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[update(guard = "caller_is_admin")]
pub fn request_account_rename(
    request: RenameAccount,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> OperationId {
    let caller = ic_cdk::caller();

    let role = Role::Admin;
    let allowed_signers = with_users_can_operate(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_operation_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[update(guard = "caller_is_admin")]
pub fn request_create_account(
    request: CreateAccount,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> OperationId {
    let caller = ic_cdk::caller();

    let role = Role::Admin;
    let allowed_signers = with_users_can_operate(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_operation_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[update(guard = "caller_is_admin")]
pub fn request_delete_account(
    request: RemoveAccount,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> OperationId {
    let caller = ic_cdk::caller();

    let role = Role::Admin;
    let allowed_signers = with_users_can_operate(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_operation_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[update(guard = "caller_is_admin")]
pub fn request_transfer_icp(
    request: IcpTransfer,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> OperationId {
    let caller = ic_cdk::caller();

    let role = Role::Admin;
    let allowed_signers = with_users_can_operate(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_operation_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[update(guard = "caller_is_admin")]
pub fn request_transfer_btc(
    request: BtcTransfer,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> OperationId {
    let caller = ic_cdk::caller();

    let role = Role::Admin;
    let allowed_signers = with_users_can_operate(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_operation_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[update(guard = "caller_is_signer")]
pub fn request_send(
    request: SendToken,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> OperationId {
    let caller = ic_cdk::caller();

    request.validate_request().unwrap_or_else(revert);

    let role = Role::Admin;
    let allowed_signers = with_users_can_operate(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_operation_mut(|s| {
        let new_request = s.new_request(caller, request_args);

        s.insert_new_request(new_request)
    })
}

#[update(guard = "caller_is_signer")]
pub async fn request_upgrade_canister(wasm_version: String) -> OperationId {
    let caller = ic_cdk::caller();

    let upgrade_request = with_wasm(|w| UpgradeCanister {
        wasm_hash_string: w.generate_hash_string(),
        wasm_version,
    });

    upgrade_request.validate_request().unwrap_or_else(revert);

    let role = Role::Admin;
    let allowed_signers = with_users_can_operate(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        role,
        allowed_signers,
        request: upgrade_request.into(),
        version: version(),
        reason: "Upgrade canister".to_string(),
        deadline: None,
    };

    with_operation_mut(|s| {
        let new_request = s.new_request(caller, request_args);

        s.insert_new_request(new_request)
    })
}
