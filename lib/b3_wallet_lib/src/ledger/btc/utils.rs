use bitcoin::{PublicKey, Script, ScriptBuf};

pub fn mock_signer(public_key: &PublicKey) -> ScriptBuf {
    // Add signature and public key to script
    let script = Script::builder()
        .push_slice(&[0; 72])
        .push_key(&public_key)
        .into_script();

    script
}
