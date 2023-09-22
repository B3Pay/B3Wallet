use ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument;

pub const HTTPS_OUTCALL_BASE_COST: u128 = 49_140_000;
pub const HTTPS_OUTCALL_REQ_COST_PER_BYTE: u128 = 5200;
pub const HTTPS_OUTCALL_RESP_COST_PER_BYTE: u128 = 10_400;

pub struct HttpsOutcallCost;

impl HttpsOutcallCost {
    /// Calculate the total cost for an HTTPS outcall
    pub fn total(arg: &CanisterHttpRequestArgument) -> u128 {
        let max_resp_bytes = Self::max_resp_bytes(arg);
        let enc_arg_size = Self::enc_arg_size(arg);

        HTTPS_OUTCALL_BASE_COST
            + enc_arg_size * HTTPS_OUTCALL_REQ_COST_PER_BYTE
            + max_resp_bytes * HTTPS_OUTCALL_RESP_COST_PER_BYTE
    }

    /// Get the maximum response bytes, defaulting to 2 MiB if not provided
    pub fn max_resp_bytes(arg: &CanisterHttpRequestArgument) -> u128 {
        arg.max_response_bytes
            .map_or(2 * 1024 * 1024, |n| n as u128)
    }

    /// Get the size of the encoded arguments
    pub fn enc_arg_size(arg: &CanisterHttpRequestArgument) -> u128 {
        let arg_raw = candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");
        arg_raw.len() as u128 + "http_request".len() as u128
    }
}
