use candid::{candid_method, CandidType, Principal};
use http::{HttpRequest, HttpResponse, HttpResponseBuilder};
use ic_cdk::api::call::{CallResult, RejectionCode};
use ic_cdk::api::caller as caller_api;
use ic_cdk::storage;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;

mod types;
use types::*;
mod http;

const VERIFY_KEY: [u8; 96] = [
    148, 21, 69, 193, 240, 64, 108, 58, 60, 123, 70, 9, 217, 16, 60, 228, 83, 52, 196, 161, 181,
    228, 201, 167, 155, 137, 140, 203, 86, 105, 79, 210, 201, 98, 15, 186, 240, 172, 64, 135, 70,
    29, 144, 59, 242, 65, 212, 247, 20, 8, 172, 152, 10, 41, 97, 107, 23, 150, 121, 50, 58, 177,
    93, 196, 13, 211, 174, 54, 60, 166, 229, 253, 3, 55, 221, 46, 93, 108, 82, 187, 105, 34, 239,
    39, 28, 132, 165, 20, 127, 61, 130, 58, 157, 132, 52, 148,
];

type PrincipalName = String;

/// Deriving CandidType or implementing it is necessary for
/// almost everything IC - if you want your structs to
/// Save in stable storage or serialize in inputs/outputs
/// You should derive CandidType, Serialize, Deserialize.
#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct EncryptedNote {
    id: u128,
    encrypted_text: String,
}

/// There can only be one Type in stable storage at a time.
/// We use this struct to represent the full CanisterState
/// So we can serialize it to stable storage.
#[derive(Clone, CandidType, Serialize, Deserialize)]
struct CanisterState {
    // During canister upgrades, this field contains a stable representation of the value stored in [NEXT_NOTE]
    counter: u128,
    // We use a BTreeMap vice a HashMap for deterministic ordering.
    notes: BTreeMap<u128, EncryptedNote>,
    // We use a BTreeMap vice a HashMap for deterministic ordering.
    user_notes: BTreeMap<PrincipalName, Vec<u128>>,
}

// WASM is single-threaded by nature. [RefCell] and [thread_local!] are used despite being not totally safe primitives.
// This is to ensure that the canister state can be used throughout.
// Your other option here is to avoid [thread_local!] and use a [RefCell<RwLock/Mutex/Atomic>].
// Here we use [thread_local!] because it is simpler.
thread_local! {

    // Currently, a single canister smart contract is limited to 4 GB of storage due to WebAssembly limitations.
    // To ensure that our canister does not exceed this limit, we restrict memory usage to at most 2 GB because
    // up to 2x memory may be needed for data serialization during canister upgrades. Therefore, we aim to support
    // up to 1,000 users, each storing up to 2 MB of data.
    // The data is reserved for storing the notes:
    //     NOTES_PER_USER = MAX_NOTES_PER_USER x MAX_NOTE_CHARS x (4 bytes per char)
    //     2 MB = 500 x 1000 x 4 = 2,000,000

    // Define dapp limits - important for security assurance
    static MAX_USERS: usize = 1_000;
    static MAX_NOTES_PER_USER: usize = 500;
    static MAX_NOTE_CHARS: usize = 1000;

    pub static NEXT_NOTE: RefCell<u128> = RefCell::new(1);
    pub static NOTES: RefCell<BTreeMap<u128, EncryptedNote>> = RefCell::new(BTreeMap::new());
    pub static USER_NOTES: RefCell<BTreeMap<PrincipalName, Vec<u128>>> = RefCell::new(BTreeMap::new());
    pub static USER_VETKD: RefCell<BTreeMap<PrincipalName, Vec<u8>>> = RefCell::new(BTreeMap::new());
}

/// Unlike Motoko, the caller identity is not built into Rust.
/// Thus, we use the ic_cdk::api::caller() method inside this wrapper function.
/// The wrapper prevents the use of the anonymous identity. Forbidding anonymous
/// interactions is the recommended default behavior for IC canisters.
fn caller() -> Principal {
    let caller = caller_api();
    // The anonymous principal is not allowed to interact with the
    // encrypted notes canister.
    // if caller == Principal::anonymous() {
    //     panic!("Anonymous principal not allowed to make calls.")
    // }
    caller
}

#[init]
fn init() {}

/// --- Queries vs. Updates ---
///
/// Note that our public methods are declared as an *updates* rather than *queries*, e.g.:
/// #[update(name = "notesCnt")] ...
/// rather than
/// #[query(name = "notesCnt")] ...
///
/// While queries are significantly faster than updates, they are not certified by the IC.
/// Thus, we avoid using queries throughout this dapp, ensuring that the result of our
/// methods gets through consensus. Otherwise, this method could e.g. omit some notes
/// if it got executed by a malicious node. (To make the dapp more efficient, one could
/// use an approach in which both queries and updates are combined.)
///
/// See https://internetcomputer.org/docs/current/concepts/canisters-code#query-and-update-methods

/// Reflects the [caller]'s identity by returning (a future of) its principal.
/// Useful for debugging.
#[query(name = "whoami")]
#[candid_method(query)]
fn whoami() -> String {
    caller_api().to_string()
}

/// General assumptions
/// -------------------
/// All the functions of this canister's public API should be available only to
/// registered users, with the exception of [register_device] and [whoami].

/// Returns the current number of users.
fn user_count() -> usize {
    NOTES.with(|notes_ref| notes_ref.borrow().keys().len())
}

/// Check that a note identifier is sane. This is needed since we use finite-
/// precision integers (`u128`).
fn is_id_sane(id: u128) -> bool {
    MAX_NOTES_PER_USER
        .with(|max_notes_per_user| id < (*max_notes_per_user as u128) * (user_count() as u128))
}

/// Returns (a future of) this [caller]'s notes.
/// Panics:
///     [caller] is the anonymous identity
///     [caller] is not a registered user
#[query(name = "get_user_notes")]
#[candid_method(query)]
fn get_user_notes() -> Vec<EncryptedNote> {
    let user = caller();
    let user_str = user.to_string();

    let note_ids = USER_NOTES.with(|notes_ref| {
        let reader = notes_ref.borrow();
        reader.get(&user_str).cloned().unwrap_or_default()
    });

    NOTES.with(|notes_ref| {
        let reader = notes_ref.borrow();
        note_ids
            .iter()
            .map(|note_id| reader.get(note_id).unwrap())
            .cloned()
            .collect()
    })
}

#[query(name = "get_notes")]
#[candid_method(query)]
fn get_notes() -> Vec<EncryptedNote> {
    NOTES.with(|notes_ref| {
        let reader = notes_ref.borrow();
        reader.values().cloned().collect()
    })
}

#[query(name = "get_note")]
#[candid_method(query)]
fn get_note(id: u128) -> Option<EncryptedNote> {
    NOTES.with(|notes_ref| {
        let reader = notes_ref.borrow();
        reader.get(&id).cloned()
    })
}

/// Delete this [caller]'s note with given id. If none of the
/// existing notes have this id, do nothing.
/// [id]: the id of the note to be deleted
///
/// Returns:
///      Future of unit
/// Panics:
///      [caller] is the anonymous identity
///      [caller] is not a registered user
///      [id] is unreasonable; see [is_id_sane]
#[update(name = "delete_note")]
#[candid_method(update)]
fn delete_note(note_id: u128) {
    let user = caller();
    assert!(is_id_sane(note_id));

    let user_str = user.to_string();
    USER_NOTES.with(|notes_ref| {
        let mut writer = notes_ref.borrow_mut();
        writer
            .get_mut(&user_str)
            .expect("user not found")
            .retain(|id| *id != note_id);
    });

    // shared ownership borrowing
    NOTES.with(|notes_ref| {
        let mut writer = notes_ref.borrow_mut();
        writer.remove(&note_id);
    });
}

/// Returns (a future of) this [caller]'s notes.
/// Panics:
///     [caller] is the anonymous identity
///     [caller] is not a registered user
///     [note.encrypted_text] exceeds [MAX_NOTE_CHARS]
///     [note.id] is unreasonable; see [is_id_sane]
#[update(name = "update_note")]
#[candid_method(update)]
fn update_note(note: EncryptedNote) {
    assert!(note.encrypted_text.chars().count() <= MAX_NOTE_CHARS.with(|mnc| *mnc));
    assert!(is_id_sane(note.id));

    NOTES.with(|notes_ref| {
        let mut writer = notes_ref.borrow_mut();
        writer.insert(note.id, note);
    });
}

/// Add new note for this [caller].
///      [note]: (encrypted) content of this note
///
/// Returns:
///      Future of unit
/// Panics:
///      [caller] is the anonymous identity
///      [caller] is not a registered user
///      [note] exceeds [MAX_NOTE_CHARS]
///      User already has [MAX_NOTES_PER_USER] notes
///      [note] would be for a new user and [MAX_USERS] is exceeded
#[update(name = "add_note")]
#[candid_method(update)]
fn add_note(note: String) {
    let user = caller();
    assert!(note.chars().count() <= MAX_NOTE_CHARS.with(|mnc| *mnc));

    let user_str = user.to_string();
    let note_id = NEXT_NOTE.with(|counter_ref| {
        let mut writer = counter_ref.borrow_mut();
        *writer += 1;
        *writer
    });

    NOTES.with(|notes_ref| {
        let mut writer = notes_ref.borrow_mut();
        writer.insert(
            note_id,
            EncryptedNote {
                id: note_id,
                encrypted_text: note,
            },
        );
    });

    USER_NOTES.with(|notes_ref| {
        let mut writer = notes_ref.borrow_mut();
        let user_notes = writer.entry(user_str).or_insert_with(Vec::new);

        assert!(user_notes.len() < MAX_NOTES_PER_USER.with(|mnpu| mnpu.clone()));

        user_notes.push(note_id);
    });
}

/// Hooks in these macros will produce a `function already defined` error
/// if they share the same name as the underlying function.

#[pre_upgrade]
/// The pre_upgrade hook determines anything your canister
/// should do before it goes offline for a code upgrade.
fn pre_upgrade() {
    let copied_counter: u128 = NEXT_NOTE.with(|counter_ref| {
        let reader = counter_ref.borrow();
        *reader
    });
    let copied_notes: BTreeMap<u128, EncryptedNote> = NOTES.with(|notes_ref| {
        let reader = notes_ref.borrow();
        reader.clone()
    });

    let copied_user_notes: BTreeMap<PrincipalName, Vec<u128>> = USER_NOTES.with(|notes_ref| {
        let reader = notes_ref.borrow();
        reader.clone()
    });

    let states = CanisterState {
        counter: copied_counter,
        notes: copied_notes,
        user_notes: copied_user_notes,
    };

    // storage::stable_save is how to write your canister state out to stable memory
    // The unwrap here is safe because the only way to get a Err back from storage::stable_save
    // is if the stable memory is full, which is not possible because we have a 4GB limit
    // on stable memory and we are only using 2GB.
    storage::stable_save((states,)).unwrap();
}

#[post_upgrade]
/// The post_upgrade hook determines anything your canister should do after it restarts
fn post_upgrade() {
    // storage::stable_restore is how to read your canister state back in from stable memory
    // Same thing with the unwrap here. For this canister there's nothing to do
    // in the event of a memory read out/in failure.
    let (old_state,): (CanisterState,) = storage::stable_restore().unwrap();

    USER_NOTES.with(|notes_ref| {
        *notes_ref.borrow_mut() = old_state.user_notes.clone();
    });

    NOTES.with(|notes_ref| {
        NEXT_NOTE.with(|counter_ref| {
            *notes_ref.borrow_mut() = old_state.notes;
            *counter_ref.borrow_mut() = old_state.counter;
        });
    });
}

const VETKD_SYSTEM_API_CANISTER_ID: &str = "br5f7-7uaaa-aaaaa-qaaca-cai";

/// Results can be cached.
#[update]
#[candid_method(update)]
async fn app_vetkd_public_key(derivation_path: Vec<Vec<u8>>) -> String {
    let request = VetKDPublicKeyRequest {
        canister_id: None,
        derivation_path,
        key_id: bls12_381_test_key_1(),
    };

    let (response,): (VetKDPublicKeyReply,) = ic_cdk::api::call::call(
        vetkd_system_api_canister_id(),
        "vetkd_public_key",
        (request,),
    )
    .await
    .expect("call to vetkd_public_key failed");

    hex::encode(response.public_key)
}

#[query]
#[candid_method(query)]
async fn symmetric_key_verification_key() -> String {
    hex::encode(VERIFY_KEY)
}

#[update]
#[candid_method(update)]
async fn encrypted_symmetric_key_for_caller(encryption_public_key: Vec<u8>) -> String {
    let request = VetKDEncryptedKeyRequest {
        derivation_id: ic_cdk::caller().as_slice().to_vec(),
        public_key_derivation_path: vec![b"symmetric_key".to_vec()],
        key_id: bls12_381_test_key_1(),
        encryption_public_key,
    };

    let (response,): (VetKDEncryptedKeyReply,) = ic_cdk::api::call::call(
        vetkd_system_api_canister_id(),
        "vetkd_encrypted_key",
        (request,),
    )
    .await
    .expect("call to vetkd_encrypted_key failed");

    hex::encode(response.encrypted_key)
}

#[query]
#[candid_method(query)]
async fn get_user_using_symmetric_key(
    encrypted_symmetric_key_string: String,
) -> CallResult<String> {
    let encrypted_symmetric_key = hex::decode(encrypted_symmetric_key_string)
        .map_err(|_| (RejectionCode::CanisterError, "Invalid hex".to_string()))?;

    USER_VETKD
        .with(|notes_ref| {
            let reader = notes_ref.borrow();
            reader
                .iter()
                .find(|(_, encrypted_symmetric_key2)| {
                    encrypted_symmetric_key == **encrypted_symmetric_key2
                })
                .map(|(principal_name, _)| principal_name)
                .cloned()
        })
        .ok_or_else(|| {
            (
                RejectionCode::CanisterError,
                "No user found for encrypted symmetric key".to_string(),
            )
        })
}

#[query]
#[candid_method(query)]
async fn encrypted_symmetric_key_for_caller_string() -> CallResult<String> {
    USER_VETKD
        .with(|notes_ref| {
            let reader = notes_ref.borrow();
            reader
                .get(&caller_api().to_string())
                .map(|encrypted_symmetric_key| hex::encode(encrypted_symmetric_key))
        })
        .ok_or_else(|| {
            (
                RejectionCode::CanisterError,
                "No key found for caller".to_string(),
            )
        })
}

#[query(name = "encryptedSymmetricKeyForCaller")]
#[candid_method(query, rename = "encryptedSymmetricKeyForCaller")]
async fn encrypted_symmetric_key_for_caller2() -> CallResult<Vec<u8>> {
    USER_VETKD
        .with(|notes_ref| {
            let reader = notes_ref.borrow();
            reader.get(&caller_api().to_string()).cloned()
        })
        .ok_or_else(|| {
            (
                RejectionCode::CanisterError,
                "No key found for caller".to_string(),
            )
        })
}

#[update(name = "setEncryptedSymmetricKeyForCaller")]
#[candid_method(update, rename = "setEncryptedSymmetricKeyForCaller")]
async fn set_encrypted_symmetric_key_for_caller(encrypted_symmetric_key: Vec<u8>) {
    USER_VETKD.with(|notes_ref| {
        let mut writer = notes_ref.borrow_mut();
        writer.insert(caller_api().to_string(), encrypted_symmetric_key);
    });
}

fn bls12_381_test_key_1() -> VetKDKeyId {
    VetKDKeyId {
        curve: VetKDCurve::Bls12_381,
        name: "test_key_1".to_string(),
    }
}

fn vetkd_system_api_canister_id() -> CanisterId {
    use std::str::FromStr;
    CanisterId::from_str(VETKD_SYSTEM_API_CANISTER_ID).expect("failed to create canister ID")
}

#[query]
#[candid_method(query)]
fn http_request(req: HttpRequest) -> HttpResponse {
    if req.path() == "/login" {
        let body = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Password Encryption</title>
    </head>
    <body>
        <h1>Enter Password</h1>
        <input type="password" id="password" />
        <button onclick="encrypt()">Encrypt</button>
        <script>
            async function encrypt() {
                const password = document.getElementById('password').value;

                const encoder = new TextEncoder();
                const data = encoder.encode(password);

                const keyData = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

                const key = await window.crypto.subtle.importKey(
                    'raw',
                    keyData,
                    'AES-GCM',
                    false,
                    ['encrypt', 'decrypt']
                );
                const iv = window.crypto.getRandomValues(new Uint8Array(12));
                const encryptedData = await window.crypto.subtle.encrypt(
                    {
                        name: 'AES-GCM',
                        iv: iv
                    },
                    key,
                    data
                );
                const encryptedBase64 = btoa(String.fromCharCode(...new Uint8Array(encryptedData)));

                console.log('Encrypted Password:', encryptedBase64);

                // Send the encrypted password to the Rust endpoint
                const response = await fetch('/handle_encrypted_password', {
                    method: 'POST',
                    body: JSON.stringify({ encryptedPassword: encryptedBase64 }),
                    headers: {
                        'Content-Type': 'application/json'
                    }
                });
            
                // Handle the response from the Rust endpoint
                const result = await response.text();
                console.log('Server Response:', result);
            }
        </script>
    </body>
    </html>
    "#;

        HttpResponseBuilder::ok()
            .header("Content-Type", "text/html; charset=utf-8")
            .with_body_and_content_length(body)
            .build()
    } else if req.path() == "/handle_encrypted_password" {
        let body = req.body_as_text();

        HttpResponseBuilder::ok()
            .header("Content-Type", "text/html; charset=utf-8")
            .with_body_and_content_length(body)
            .build()
    } else {
        HttpResponseBuilder::not_found().build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::export_service;

    #[test]
    fn test_user_count_succeeds() {
        assert_eq!(user_count(), 0);
    }

    #[test]
    fn generate_candid() {
        use std::io::Write;

        let mut file = std::fs::File::create("./candid.did").unwrap();

        export_service!();

        let candid = __export_service();

        file.write_all(candid.as_bytes()).unwrap();

        assert!(true);
    }
}
