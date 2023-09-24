use b3_utils::ledger::raw_keccak256;
use bitcoin::secp256k1::PublicKey;

use super::{error::EvmError, types::PublicKeyTrait};

pub fn get_method_id(method_sig: &str) -> String {
    let result = raw_keccak256(method_sig.as_bytes()).to_hex_string();

    let hex_string = result[..8].to_string();

    hex_string
}

pub fn get_transfer_data(address: &str, amount: u64) -> Result<String, EvmError> {
    if address.len() != 42 {
        return Err(EvmError::InvalidAddress(address.to_string()));
    }
    let method_sig = "transfer(address,uint256)";

    let method_id = get_method_id(method_sig);

    let address_64 = format!("{:0>64}", &address[2..]);

    let amount_hex = format!("{:02x}", amount);
    let amount_64 = format!("{:0>64}", amount_hex);

    Ok(method_id + &address_64 + &amount_64)
}

pub fn create_address_from(public_key: &PublicKey, nonce: u64) -> String {
    let sender = public_key.to_evm_key();

    let mut stream = rlp::RlpStream::new_list(2);
    stream.append(&sender);
    stream.append(&nonce);

    let rlp_encoded = stream.out();

    let hash = raw_keccak256(&rlp_encoded).to_hex_string();

    // Grab the right-most 20 bytes
    let address = "0x".to_string() + &hash[24..];

    address
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
    fn test_get_method_id() {
        let method_sig = "transfer(address,uint256)";

        let expected_result = "a9059cbb";

        let result = get_method_id(method_sig);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_get_transfer_data() {
        let address = "0x7a9d2f53fea15e31f0a89d7f5d9e0e82b0b88ad6";
        let amount = 12345;

        let expected_result = "a9059cbb0000000000000000000000007a9d2f53fea15e31f0a89d7f5d9e0e82b0b88ad60000000000000000000000000000000000000000000000000000000000003039";

        let result = get_transfer_data(address, amount).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_create_address_from() {
        let pub_key =
            string_to_vec_u8("02c397f23149d3464517d57b7cdc8e287428407f9beabfac731e7c24d536266cd1");

        println!("pub_key: {:?}", pub_key);

        let public_key = PublicKey::from_slice(&pub_key).unwrap();

        let pub_key = public_key.serialize_uncompressed();

        let pub_key_hash = raw_keccak256(&pub_key[1..]).to_hex_string();

        let sender = "0x".to_string() + &pub_key_hash[24..];

        let expected_address = "0x907dc4d0be5d691970cae886fcab34ed65a2cd66";

        println!("sender: {:?}", sender);

        assert_eq!(sender, expected_address);

        let nonce = 0;

        let expected_result = "0x0407316cb70d5a7d4642b592e9cb37fa70c56cd1";

        let result = create_address_from(&public_key, nonce);

        assert_eq!(result, expected_result);

        let nonce = 1;

        let expected_result = "0xa871c4b1dc678be80af6b5cc8aa4910ad62b11cb";

        let result = create_address_from(&public_key, nonce);

        assert_eq!(result, expected_result);
    }
}
