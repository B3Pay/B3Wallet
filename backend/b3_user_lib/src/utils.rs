use easy_hasher::easy_hasher;

pub fn get_address_from_public_key(public_key: Vec<u8>) -> Result<String, String> {
    if public_key.len() != 33 {
        return Err("Invalid length of public key".to_string());
    }

    let pub_key_arr: [u8; 33] = public_key[..].try_into().unwrap();
    let pub_key = libsecp256k1::PublicKey::parse_compressed(&pub_key_arr)
        .map_err(|e| format!("{}", e))?
        .serialize();

    let keccak256 = easy_hasher::raw_keccak256(pub_key[1..].to_vec());
    let keccak256_hex = keccak256.to_hex_string();
    let address: String = "0x".to_owned() + &keccak256_hex[24..];

    Ok(address)
}
#[cfg(test)]
pub fn get_transfer_data(address: &str, amount: u64) -> Result<String, String> {
    if address.len() != 42 {
        return Err("Invalid address".to_string());
    }
    let method_sig = "transfer(address,uint256)";
    let keccak256 = easy_hasher::raw_keccak256(method_sig.as_bytes().to_vec());
    let method_id = &keccak256.to_hex_string()[..8];

    let address_64 = format!("{:0>64}", &address[2..]);

    let amount_hex = format!("{:02x}", amount);
    let amount_64 = format!("{:0>64}", amount_hex);

    Ok(method_id.to_owned() + &address_64 + &amount_64)
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
    fn get_address_from_public_key_valid() {
        let expected = "0x907dc4d0be5d691970cae886fcab34ed65a2cd66";
        let public_key_str = "02c397f23149d3464517d57b7cdc8e287428407f9beabfac731e7c24d536266cd1";
        let public_key_to_vec = string_to_vec_u8(&public_key_str);
        let result = get_address_from_public_key(public_key_to_vec).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn get_address_from_public_with_zeros() {
        let expected = Err("Invalid public key".to_string());
        let public_key_str = "000000000000000000000000000000000000000000000000000000000000000000";
        let public_key_to_vec = string_to_vec_u8(&public_key_str);
        let result = get_address_from_public_key(public_key_to_vec);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_address_from_public_with_empty_public_key() {
        let expected = Err("Invalid length of public key".to_string());
        let public_key_str = "";
        let public_key_to_vec = string_to_vec_u8(&public_key_str);
        let result = get_address_from_public_key(public_key_to_vec);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_transfer_data_valid() {
        let expected ="a9059cbb000000000000000000000000907dc4d0be5d691970cae886fcab34ed65a2cd660000000000000000000000000000000000000000000000000000000000000001";

        let address = "0x907dc4d0be5d691970cae886fcab34ed65a2cd66";
        let amount = 1;
        let result = get_transfer_data(address, amount).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn get_transfer_data_with_invalid_address() {
        let expected = Err("Invalid address".to_string());
        let address = "0x00";
        let value = 1;
        let result = get_transfer_data(address, value);
        assert_eq!(result, expected);
    }
}
