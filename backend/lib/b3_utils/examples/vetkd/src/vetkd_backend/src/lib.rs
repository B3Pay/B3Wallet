use b3_utils::{
    log,
    vetkd::{VetKD, VetKDManagement},
};
use ic_cdk::update;

#[update]
async fn symmetric_key_verification_key() -> String {
    let public_key = VetKDManagement(ic_cdk::id())
        .request_public_key(vec![b"symmetric_key".to_vec()])
        .await
        .expect("call to vetkd_encrypted_key failed");

    hex::encode(public_key)
}

#[update]
async fn encrypted_symmetric_key_for_caller(encryption_public_key: Vec<u8>) -> String {
    debug_println_caller("encrypted_symmetric_key_for_caller");

    let encrypted_key = VetKD(ic_cdk::caller().into())
        .request_encrypted_key(vec![b"symmetric_key".to_vec()], encryption_public_key)
        .await
        .expect("call to vetkd_encrypted_key failed");

    hex::encode(encrypted_key)
}

#[update]
async fn ibe_encryption_key() -> String {
    let public_key = VetKDManagement(ic_cdk::id())
        .request_public_key(vec![b"ibe_decryption".to_vec()])
        .await
        .expect("call to vetkd_encrypted_key failed");

    hex::encode(public_key)
}

#[update]
async fn encrypted_ibe_decryption_key_for_caller(encryption_public_key: Vec<u8>) -> String {
    debug_println_caller("encrypted_ibe_decryption_key_for_caller");

    let encrypted_key = VetKD(ic_cdk::caller().into())
        .request_encrypted_key(vec![b"ibe_decryption".to_vec()], encryption_public_key)
        .await
        .expect("call to vetkd_encrypted_key failed");

    hex::encode(encrypted_key)
}

fn debug_println_caller(method_name: &str) {
    log!(
        "{}: caller: {} (isAnonymous: {})",
        method_name,
        ic_cdk::caller().to_text(),
        ic_cdk::caller() == candid::Principal::anonymous()
    );
}
