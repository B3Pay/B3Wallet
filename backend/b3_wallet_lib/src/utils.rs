use crate::error::WalletError;
use easy_hasher::easy_hasher;

pub fn get_transfer_data(address: &str, amount: u64) -> Result<String, WalletError> {
    if address.len() != 42 {
        return Err(WalletError::InvalidAddress);
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

// Converts a SEC1 ECDSA signature to the DER format.
pub fn sec1_to_der(sec1_signature: Vec<u8>) -> Vec<u8> {
    let r: Vec<u8> = if sec1_signature[0] & 0x80 != 0 {
        // r is negative. Prepend a zero byte.
        let mut tmp = vec![0x00];
        tmp.extend(sec1_signature[..32].to_vec());
        tmp
    } else {
        // r is positive.
        sec1_signature[..32].to_vec()
    };

    let s: Vec<u8> = if sec1_signature[32] & 0x80 != 0 {
        // s is negative. Prepend a zero byte.
        let mut tmp = vec![0x00];
        tmp.extend(sec1_signature[32..].to_vec());
        tmp
    } else {
        // s is positive.
        sec1_signature[32..].to_vec()
    };

    // Convert signature to DER.
    vec![
        vec![0x30, 4 + r.len() as u8 + s.len() as u8, 0x02, r.len() as u8],
        r,
        vec![0x02, s.len() as u8],
        s,
    ]
    .into_iter()
    .flatten()
    .collect()
}
