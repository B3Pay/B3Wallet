use crate::error::HelperError;

pub fn vec_to_hex_string<V: AsRef<[u8]>>(data: V) -> String {
    hex::encode(data)
}

pub fn vec_to_hex_string_with_0x<V: AsRef<[u8]>>(data: V) -> String {
    format!("0x{}", vec_to_hex_string(data))
}

pub fn hex_string_to_vec<S: AsRef<str>>(stringlike: S) -> Result<Vec<u8>, HelperError> {
    let str_ref = stringlike.as_ref();

    hex::decode(str_ref).map_err(|e| HelperError::HexStringToVecError(e.to_string()))
}

pub fn hex_string_to_vec_without_0x<S: AsRef<str>>(stringlike: S) -> Result<Vec<u8>, HelperError> {
    let str_ref = stringlike.as_ref();

    hex_string_to_vec(str_ref.trim_start_matches("0x"))
}
