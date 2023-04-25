use crate::types::{ECDSAPublicKeyResponse, SignWithECDSAArgs, SignWithECDSAResponse};
use crate::utils::string_to_vec_u8;
use candid::de::IDLDeserialize;
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use candid::{Decode, Encode};
use ic_cdk::api::call::{CallResult, RejectionCode};
use ic_cdk::export::Principal;
use libsecp256k1::{PublicKey, SecretKey};
use std::cell::RefCell;
use std::future::Future;

#[derive(Clone, Default)]
struct StateTest {
    private_key: String,
}

thread_local! {
    static STATE_TEST: RefCell<StateTest> = RefCell::new(StateTest::default());
}

pub fn ic_timestamp() -> u64 {
    u64::from(1667817318 as u64)
}

pub fn ic_call<T: ArgumentEncoder, R: for<'a> ArgumentDecoder<'a>>(
    _id: Principal,
    method: &str,
    args: T,
    _cycles: u64,
) -> impl Future<Output = CallResult<R>> + '_ {
    let args_raw = candid::encode_args(args).expect("Failed to encode arguments.");

    async move {
        if method == "ecdsa_public_key" {
            let private_key_state = STATE_TEST.with(|s| {
                let mut state = s.borrow_mut();
                state.private_key = String::from(
                    "5c86d3784f39013aa50aada6d97f9bad733636d57bf6bb18b0bca1ffcff374b4",
                );
                state.private_key.clone()
            });

            let private_key =
                SecretKey::parse_slice(&string_to_vec_u8(&private_key_state)).unwrap();

            let public_key = PublicKey::from_secret_key(&private_key).serialize_compressed();

            let res = ECDSAPublicKeyResponse {
                public_key: public_key.to_vec(),
                chain_code: vec![0, 1],
            };

            let bytes = Encode!(&res).unwrap();
            let mut de = IDLDeserialize::new(&bytes).unwrap();
            let res_decoded: R = ArgumentDecoder::decode(&mut de).unwrap();

            return Ok(res_decoded);
        }
        if method == "sign_with_ecdsa" {
            let private_key_state = STATE_TEST.with(|s| s.borrow().private_key.clone());
            let private_key =
                SecretKey::parse_slice(&string_to_vec_u8(&private_key_state)).unwrap();

            let args = Decode!(&args_raw, SignWithECDSAArgs).unwrap();

            let message: [u8; 32] = args.message_hash[..32].try_into().unwrap();

            let message_parsed = libsecp256k1::Message::parse(&message);

            let signature = libsecp256k1::sign(&message_parsed, &private_key);

            let res = SignWithECDSAResponse {
                signature: signature.0.serialize().to_vec(),
            };
            let bytes = Encode!(&res).unwrap();
            let mut de = IDLDeserialize::new(&bytes).unwrap();
            let res_decoded: R = ArgumentDecoder::decode(&mut de).unwrap();

            return Ok(res_decoded);
        } else {
            return Err((RejectionCode::CanisterReject, String::from("no method")));
        }
    }
}
