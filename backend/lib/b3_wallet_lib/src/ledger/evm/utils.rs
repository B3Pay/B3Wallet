use crate::error::WalletError;
use b3_helper_lib::sha3_sha256;

pub fn get_method_id(method_sig: &str) -> String {
    let result = sha3_sha256(method_sig.as_bytes());

    let hex_string = result
        .iter()
        .take(4)
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    hex_string
}

pub fn get_transfer_data(address: &str, amount: u64) -> Result<String, WalletError> {
    if address.len() != 42 {
        return Err(WalletError::InvalidAddress);
    }
    let method_sig = "transfer(address,uint256)";

    let method_id = get_method_id(method_sig);

    let address_64 = format!("{:0>64}", &address[2..]);

    let amount_hex = format!("{:02x}", amount);
    let amount_64 = format!("{:0>64}", amount_hex);

    Ok(method_id + &address_64 + &amount_64)
}

pub fn string_to_vec_u8(str: &str) -> Vec<u8> {
    let starts_from: usize;
    if str.starts_with("0x") {
        starts_from = 2;
    } else {
        starts_from = 0;
    }

    (starts_from..str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>()
}

pub fn remove_leading(vec: Vec<u8>, element: u8) -> Vec<u8> {
    let start = vec.iter().position(|&x| x != element).unwrap();
    let result = &vec[start..];
    result.to_vec()
}

pub fn u64_to_vec_u8(u: &u64) -> Vec<u8> {
    u.to_be_bytes()
        .into_iter()
        .skip_while(|&x| x == 0)
        .collect()
}

pub fn vec_u8_to_string(vec: &Vec<u8>) -> String {
    vec.iter()
        .map(|r| format!("{:02x}", r))
        .collect::<Vec<String>>()
        .join("")
        .to_string()
}

pub fn vec_u8_to_u64(vec: &Vec<u8>) -> u64 {
    let mut _vec = [0; 8];
    _vec[8 - vec.len()..].copy_from_slice(&vec);
    u64::from_be_bytes(_vec).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_transfer_data() {
        let address = "0x7a9d2f53fea15e31f0a89d7f5d9e0e82b0b88ad6";
        let amount = 12345;

        let expected_result = "4b40e9010000000000000000000000007a9d2f53fea15e31f0a89d7f5d9e0e82b0b88ad60000000000000000000000000000000000000000000000000000000000003039";

        let result = get_transfer_data(address, amount).unwrap();

        assert_eq!(result, expected_result);
    }
}
