use crate::utils::{ remove_leading, string_to_vec_u8, u64_to_vec_u8, vec_u8_to_string, vec_u8_to_u64 };
use easy_hasher::easy_hasher;

#[derive(Debug, Clone, PartialEq)]
enum TransactionType {
    Legacy,
    EIP1559,
    EIP2930,
}

pub trait Sign {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String>;
    fn sign(&mut self, signature: Vec<u8>, public_key: Vec<u8>) -> Result<Vec<u8>, String>;
    fn is_signed(&self) -> bool;
    fn get_signature(&self) -> Result<Vec<u8>, String>;
    fn get_recovery_id(&self) -> Result<u8, String>;
    fn get_nonce(&self) -> Result<u64, String>;
    fn serialize(&self) -> Result<Vec<u8>, String>;
}

pub struct TransactionLegacy {
    pub chain_id: u64,
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub to: String,
    pub value: u64,
    pub data: String,
    pub v: String,
    pub r: String,
    pub s: String,
}
impl From<(Vec<u8>, u64)> for TransactionLegacy {
    fn from(data: (Vec<u8>, u64)) -> Self {
        let rlp = rlp::Rlp::new(&data.0[..]);

        let nonce_hex = rlp.at(0).as_val::<Vec<u8>>();
        let nonce = vec_u8_to_u64(&nonce_hex);

        let gas_price_hex = rlp.at(1).as_val::<Vec<u8>>();
        let gas_price = vec_u8_to_u64(&gas_price_hex);

        let gas_limit_hex = rlp.at(2).as_val::<Vec<u8>>();
        let gas_limit = vec_u8_to_u64(&gas_limit_hex);

        let to_hex = rlp.at(3).as_val::<Vec<u8>>();
        let to = vec_u8_to_string(&to_hex);

        let value_hex = rlp.at(4).as_val::<Vec<u8>>();
        let value = vec_u8_to_u64(&value_hex);

        let data_tx_hex = rlp.at(5).as_val::<Vec<u8>>();
        let data_tx = vec_u8_to_string(&data_tx_hex);

        let v_hex = rlp.at(6).as_val::<Vec<u8>>();
        let v = vec_u8_to_string(&v_hex);

        let r_hex = rlp.at(7).as_val::<Vec<u8>>();
        let r = vec_u8_to_string(&r_hex);

        let s_hex = rlp.at(8).as_val::<Vec<u8>>();
        let s = vec_u8_to_string(&s_hex);

        let chain_id =data.1;

        TransactionLegacy {
            chain_id,
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data: data_tx,
            v,
            r,
            s,
        }
    }
}
impl Sign for TransactionLegacy {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String> {
        let mut stream = rlp::RlpStream::new_list(9);

        let items = [
            u64_to_vec_u8(&self.nonce),
            u64_to_vec_u8(&self.gas_price),
            u64_to_vec_u8(&self.gas_limit),
            string_to_vec_u8(&self.to),
            u64_to_vec_u8(&self.value),
            string_to_vec_u8(&self.data),
            u64_to_vec_u8(&self.chain_id),
        ];

        for item in items {
            stream.append(&item);
        }

        stream.append_empty_data();
        stream.append_empty_data();

        let encoded_tx = stream.out();

        let keccak256 = easy_hasher::raw_keccak256(encoded_tx);

        Ok(keccak256.to_vec())
    }
    fn sign(&mut self, signature: Vec<u8>, public_key: Vec<u8>) -> Result<Vec<u8>, String> {
        let chain_id = u64::try_from(self.chain_id).unwrap();

        let r_remove_leading_zeros = remove_leading(signature[..32].to_vec(), 0);
        let s_remove_leading_zeros = remove_leading(signature[32..].to_vec(), 0);

        let r = vec_u8_to_string(&r_remove_leading_zeros);

        let s = vec_u8_to_string(&s_remove_leading_zeros);

        let message = self.get_message_to_sign().unwrap();
        let recovery_id = get_recovery_id(&message, &signature, &public_key).unwrap();

        let v_number = chain_id * 2 + 35 + u64::try_from(recovery_id).unwrap();
        let v = vec_u8_to_string(&u64_to_vec_u8(&v_number));

        self.v = v;
        self.r = r;
        self.s = s;

        let result = self.serialize().unwrap();
        Ok(result)
    }
    fn is_signed(&self) -> bool {
        let r: String;
        if self.r.starts_with("0x") {
            r = self.r[2..].to_string();
        } else {
            r = self.r[..].to_string();
        }
        let s: String;
        if self.s.starts_with("0x") {
            s = self.s[2..].to_string();
        } else {
            s = self.s[..].to_string();
        }

        r != "00" || s != "00"
    }
    fn get_signature(&self) -> Result<Vec<u8>, String> {
        if !self.is_signed() {
            return Err("This is not a signed transaction".to_string());
        }

        let r = string_to_vec_u8(&self.r);
        let s = string_to_vec_u8(&self.s);

        Ok([&r[..], &s[..]].concat())
    }
    fn get_recovery_id(&self) -> Result<u8, String> {
        if !self.is_signed() {
            return Err("This is not a signed transaction".to_string());
        }
        let chain_id = i64::try_from(self.chain_id).unwrap();
        let v = string_to_vec_u8(&self.v);

        let recovery_id = -1 * ((chain_id * 2) + 35 - i64::try_from(v[0]).unwrap());
        Ok(u8::try_from(recovery_id).unwrap())
    }
    fn serialize(&self) -> Result<Vec<u8>, String> {
        let mut stream = rlp::RlpStream::new_list(9);

        let nonce = u64_to_vec_u8(&self.nonce);
        stream.append(&nonce);

        let gas_price = u64_to_vec_u8(&self.gas_price);
        stream.append(&gas_price);

        let gas_limit = u64_to_vec_u8(&self.gas_limit);
        stream.append(&gas_limit);

        let to = string_to_vec_u8(&self.to[..]);
        stream.append(&to);

        let value = u64_to_vec_u8(&self.value);
        stream.append(&value);

        let data = string_to_vec_u8(&self.data[..]);
        stream.append(&data);

        let v = string_to_vec_u8(&self.v[..]);
        stream.append(&v);

        let r = string_to_vec_u8(&self.r[..]);
        stream.append(&r);

        let s = string_to_vec_u8(&self.s[..]);
        stream.append(&s);

        Ok(stream.out().to_vec())
    }
    fn get_nonce(&self) -> Result<u64, String> {
        Ok(self.nonce)
    }
}

pub struct Transaction2930 {
    pub chain_id: u64,
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub to: String,
    pub value: u64,
    pub data: String,
    pub access_list: Vec<(String, Vec<String>)>,
    pub v: String,
    pub r: String,
    pub s: String,
}
impl From<Vec<u8>> for Transaction2930 {
    fn from(data: Vec<u8>) -> Self {
        let rlp = rlp::Rlp::new(&data[1..]);

        let chain_id_hex = rlp.at(0).as_val::<Vec<u8>>();
        let chain_id = vec_u8_to_u64(&chain_id_hex);

        let nonce_hex = rlp.at(1).as_val::<Vec<u8>>();
        let nonce = vec_u8_to_u64(&nonce_hex);

        let gas_price_hex = rlp.at(2).as_val::<Vec<u8>>();
        let gas_price = vec_u8_to_u64(&gas_price_hex);

        let gas_limit_hex = rlp.at(3).as_val::<Vec<u8>>();
        let gas_limit = vec_u8_to_u64(&gas_limit_hex);

        let to_hex = rlp.at(4).as_val::<Vec<u8>>();
        let to = vec_u8_to_string(&to_hex);

        let value_hex = rlp.at(5).as_val::<Vec<u8>>();
        let value = vec_u8_to_u64(&value_hex);

        let data_tx_hex = rlp.at(6).as_val::<Vec<u8>>();
        let data_tx = vec_u8_to_string(&data_tx_hex);

        let access_list = decode_access_list(&rlp.at(7).as_raw().to_vec());

        let v_hex = rlp.at(8).as_val::<Vec<u8>>();
        let v = vec_u8_to_string(&v_hex);

        let r_hex = rlp.at(9).as_val::<Vec<u8>>();
        let r = vec_u8_to_string(&r_hex);

        let s_hex = rlp.at(10).as_val::<Vec<u8>>();
        let s = vec_u8_to_string(&s_hex);
        Transaction2930 {
            chain_id,
            nonce,
            gas_price,
            gas_limit,
            to,
            data: data_tx,
            value,
            access_list,
            v,
            r,
            s,
        }
    }
}
impl Sign for Transaction2930 {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String> {
        let mut stream = rlp::RlpStream::new_list(8);
        let items = [
            u64_to_vec_u8(&self.chain_id),
            u64_to_vec_u8(&self.nonce),
            u64_to_vec_u8(&self.gas_price),
            u64_to_vec_u8(&self.gas_limit),
            string_to_vec_u8(&self.to),
            u64_to_vec_u8(&self.value),
            string_to_vec_u8(&self.data),
        ];

        for item in items {
            stream.append(&item);
        }

        let access_list = encode_access_list(&self.access_list);
        stream.append_raw(&access_list, 1);

        let decode_tx = stream.out();

        let msg = [&[0x01], &decode_tx[..]].concat();
        let keccak256 = easy_hasher::raw_keccak256(msg);
        Ok(keccak256.to_vec())
    }
    fn sign(&mut self, signature: Vec<u8>, public_key: Vec<u8>) -> Result<Vec<u8>, String> {
        let r_remove_leading_zeros = remove_leading(signature[..32].to_vec(), 0);
        let s_remove_leading_zeros = remove_leading(signature[32..].to_vec(), 0);

        let r = vec_u8_to_string(&r_remove_leading_zeros);

        let s = vec_u8_to_string(&s_remove_leading_zeros);

        let message = self.get_message_to_sign().unwrap();
        let recovery_id = get_recovery_id(&message, &signature, &public_key).unwrap();
        let v: String;
        if recovery_id == 0 {
            v = "".to_string();
        } else {
            v = "01".to_string();
        }

        self.v = v;
        self.r = r;
        self.s = s;

        let result = self.serialize().unwrap();
        Ok(result)
    }
    fn is_signed(&self) -> bool {
        let r: String;
        if self.r.starts_with("0x") {
            r = self.r[2..].to_string();
        } else {
            r = self.r[..].to_string();
        }
        let s: String;
        if self.s.starts_with("0x") {
            s = self.s[2..].to_string();
        } else {
            s = self.s[..].to_string();
        }

        r != "00" || s != "00"
    }
    fn get_signature(&self) -> Result<Vec<u8>, String> {
        if !self.is_signed() {
            return Err("This is not a signed transaction".to_string());
        }

        let r = string_to_vec_u8(&self.r);
        let s = string_to_vec_u8(&self.s);

        Ok([&r[..], &s[..]].concat())
    }
    fn get_recovery_id(&self) -> Result<u8, String> {
        if !self.is_signed() {
            return Err("This is not a signed transaction".to_string());
        }

        let v = string_to_vec_u8(&self.v);

        if v.is_empty() {
            Ok(0 as u8)
        } else {
            Ok(1 as u8)
        }
    }
    fn serialize(&self) -> Result<Vec<u8>, String> {
        let mut stream = rlp::RlpStream::new_list(11);

        let chain_id = u64_to_vec_u8(&self.chain_id);
        stream.append(&chain_id);

        let nonce = u64_to_vec_u8(&self.nonce);
        stream.append(&nonce);

        let gas_price = u64_to_vec_u8(&self.gas_price);
        stream.append(&gas_price);

        let gas_limit = u64_to_vec_u8(&self.gas_limit);
        stream.append(&gas_limit);

        let to = string_to_vec_u8(&self.to[..]);
        stream.append(&to);

        let value = u64_to_vec_u8(&self.value);
        stream.append(&value);

        let data = string_to_vec_u8(&self.data[..]);
        stream.append(&data);

        let access_list = encode_access_list(&self.access_list);
        stream.append_raw(&access_list[..], 1);

        let v = string_to_vec_u8(&self.v[..]);
        stream.append(&v);

        let r = string_to_vec_u8(&self.r[..]);
        stream.append(&r);

        let s = string_to_vec_u8(&self.s[..]);
        stream.append(&s);

        let result = stream.out().to_vec();

        Ok([&[0x01], &result[..]].concat())
    }
    fn get_nonce(&self) -> Result<u64, String> {
        Ok(self.nonce)
    }
}

pub struct Transaction1559 {
    pub chain_id: u64,
    pub nonce: u64,
    pub max_priority_fee_per_gas: u64,
    pub gas_limit: u64,
    pub max_fee_per_gas: u64,
    pub to: String,
    pub value: u64,
    pub data: String,
    pub access_list: Vec<(String, Vec<String>)>,
    pub v: String,
    pub r: String,
    pub s: String,
}
impl From<Vec<u8>> for Transaction1559 {
    fn from(data: Vec<u8>) -> Self {
        let rlp = rlp::Rlp::new(&data[1..]);

        let chain_id_hex = rlp.at(0).as_val::<Vec<u8>>();
        let chain_id = vec_u8_to_u64(&chain_id_hex);

        let nonce_hex = rlp.at(1).as_val::<Vec<u8>>();
        let nonce = vec_u8_to_u64(&nonce_hex);

        let max_priority_fee_per_gas_hex = rlp.at(2).as_val::<Vec<u8>>();
        let max_priority_fee_per_gas = vec_u8_to_u64(&max_priority_fee_per_gas_hex);

        let max_fee_per_gas_hex = rlp.at(3).as_val::<Vec<u8>>();

        let max_fee_per_gas = vec_u8_to_u64(&max_fee_per_gas_hex);

        let gas_limit_hex = rlp.at(4).as_val::<Vec<u8>>();
        let gas_limit = vec_u8_to_u64(&gas_limit_hex);

        let to_hex = rlp.at(5).as_val::<Vec<u8>>();
        let to = vec_u8_to_string(&to_hex);

        let value_hex = rlp.at(6).as_val::<Vec<u8>>();
        let value = vec_u8_to_u64(&value_hex);

        let data_tx_hex = rlp.at(7).as_val::<Vec<u8>>();
        let data_tx = vec_u8_to_string(&data_tx_hex);

        let access_list = decode_access_list(&rlp.at(8).as_raw().to_vec());

        let v_hex = rlp.at(9).as_val::<Vec<u8>>();
        let v = vec_u8_to_string(&v_hex);

        let r_hex = rlp.at(10).as_val::<Vec<u8>>();
        let r = vec_u8_to_string(&r_hex);

        let s_hex = rlp.at(11).as_val::<Vec<u8>>();
        let s = vec_u8_to_string(&s_hex);

        Transaction1559 {
            chain_id,
            nonce,
            max_priority_fee_per_gas,
            max_fee_per_gas,
            gas_limit,
            to,
            value,
            data: data_tx,
            access_list,
            v,
            r,
            s,
        }
    }
}
impl Sign for Transaction1559 {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String> {
        let mut stream = rlp::RlpStream::new_list(9);
        let items = [
            u64_to_vec_u8(&self.chain_id),
            u64_to_vec_u8(&self.nonce),
            u64_to_vec_u8(&self.max_priority_fee_per_gas),
            u64_to_vec_u8(&self.max_fee_per_gas),
            u64_to_vec_u8(&self.gas_limit),
            string_to_vec_u8(&self.to),
            u64_to_vec_u8(&self.value),
            string_to_vec_u8(&self.data),
        ];

        for i in 0..=7 {
            let item = &items[i];
            stream.append(item);
        }

        let access_list = encode_access_list(&self.access_list);

        stream.append_raw(&access_list[..], 1);

        let decode_tx = stream.out();

        let msg = [&[0x02], &decode_tx[..]].concat();

        let keccak256 = easy_hasher::raw_keccak256(msg);

        Ok(keccak256.to_vec())
    }
    fn sign(&mut self, signature: Vec<u8>, public_key: Vec<u8>) -> Result<Vec<u8>, String> {
        let r_remove_leading_zeros = remove_leading(signature[..32].to_vec(), 0);
        let s_remove_leading_zeros = remove_leading(signature[32..].to_vec(), 0);

        let r = vec_u8_to_string(&r_remove_leading_zeros);

        let s = vec_u8_to_string(&s_remove_leading_zeros);

        let message = self.get_message_to_sign().unwrap();
        let recovery_id = get_recovery_id(&message, &signature, &public_key).unwrap();
        let v: String;
        if recovery_id == 0 {
            v = "".to_string();
        } else {
            v = "01".to_string();
        }

        self.v = v;
        self.r = r;
        self.s = s;

        let result = self.serialize().unwrap();
        Ok(result)
    }
    fn is_signed(&self) -> bool {
        let r: String;
        if self.r.starts_with("0x") {
            r = self.r[2..].to_string();
        } else {
            r = self.r[..].to_string();
        }
        let s: String;
        if self.s.starts_with("0x") {
            s = self.s[2..].to_string();
        } else {
            s = self.s[..].to_string();
        }

        r != "00" || s != "00"
    }
    fn get_signature(&self) -> Result<Vec<u8>, String> {
        if !self.is_signed() {
            return Err("This is not a signed transaction".to_string());
        }

        let r = string_to_vec_u8(&self.r);
        let s = string_to_vec_u8(&self.s);

        Ok([&r[..], &s[..]].concat())
    }
    fn get_recovery_id(&self) -> Result<u8, String> {
        if !self.is_signed() {
            return Err("This is not a signed transaction".to_string());
        }
        let v = &self.v;

        if v.is_empty() {
            Ok(0 as u8)
        } else {
            Ok(1 as u8)
        }
    }
    fn serialize(&self) -> Result<Vec<u8>, String> {
        let mut stream = rlp::RlpStream::new_list(12);

        let chain_id = u64_to_vec_u8(&self.chain_id);
        stream.append(&chain_id);

        let nonce = u64_to_vec_u8(&self.nonce);
        stream.append(&nonce);

        let max_priority_fee_per_gas = u64_to_vec_u8(&self.max_priority_fee_per_gas);
        stream.append(&max_priority_fee_per_gas);

        let max_fee_per_gas = u64_to_vec_u8(&self.max_fee_per_gas);
        stream.append(&max_fee_per_gas);

        let gas_limit = u64_to_vec_u8(&self.gas_limit);
        stream.append(&gas_limit);

        let to = string_to_vec_u8(&self.to[..]);
        stream.append(&to);

        let value = u64_to_vec_u8(&self.value);
        stream.append(&value);

        let data = string_to_vec_u8(&self.data[..]);
        stream.append(&data);

        let access_list = encode_access_list(&self.access_list);

        stream.append_raw(&access_list[..], 1);

        let v = string_to_vec_u8(&self.v[..]);
        stream.append(&v);

        let r = string_to_vec_u8(&self.r[..]);
        stream.append(&r);

        let s = string_to_vec_u8(&self.s[..]);
        stream.append(&s);

        let result = stream.out().to_vec();

        Ok([&[0x02], &result[..]].concat())
    }
    fn get_nonce(&self) -> Result<u64, String> {
        Ok(self.nonce)
    }
}

pub fn get_transaction(hex_raw_tx: &Vec<u8>, chain_id: u64) -> Result<Box<dyn Sign>, String> {
    let tx_type = get_transaction_type(hex_raw_tx).unwrap();

    if tx_type == TransactionType::Legacy {
        Ok(Box::new(TransactionLegacy::from((hex_raw_tx.clone(), chain_id))))
    } else if tx_type == TransactionType::EIP1559 {
        Ok(Box::new(Transaction1559::from(hex_raw_tx.clone())))
    } else if tx_type == TransactionType::EIP2930 {
        Ok(Box::new(Transaction2930::from(hex_raw_tx.clone())))
    } else {
        Err(String::from("Invalid type"))
    }
}

fn get_transaction_type(hex_raw_tx: &Vec<u8>) -> Result<TransactionType, String> {
    if hex_raw_tx[0] >= 0xc0 {
        Ok(TransactionType::Legacy)
    } else if hex_raw_tx[0] == 0x01 {
        Ok(TransactionType::EIP2930)
    } else if hex_raw_tx[0] == 0x02 {
        Ok(TransactionType::EIP1559)
    } else {
        Err(String::from("Invalid type"))
    }
}

fn get_recovery_id(
    message: &Vec<u8>,
    signature: &Vec<u8>,
    public_key: &Vec<u8>,
) -> Result<u8, String> {
    if signature.len() != 64 {
        return Err("Invalid signature".to_string());
    }
    if message.len() != 32 {
        return Err("Invalid message".to_string());
    }
    if public_key.len() != 33 {
        return Err("Invalid public key".to_string());
    }

    for i in 0..3 {
        let recovery_id = libsecp256k1::RecoveryId::parse_rpc(27 + i).unwrap();

        let signature_bytes: [u8; 64] = signature[..].try_into().unwrap();
        let signature_bytes_64 = libsecp256k1::Signature::parse_standard(&signature_bytes).unwrap();

        let message_bytes: [u8; 32] = message[..].try_into().unwrap();
        let message_bytes_32 = libsecp256k1::Message::parse(&message_bytes);

        let key = libsecp256k1::recover(&message_bytes_32, &signature_bytes_64, &recovery_id).unwrap();
        if key.serialize_compressed() == public_key[..] {
            return Ok(i as u8);
        }
    }
    return Err("Not found".to_string());
}

fn encode_access_list(access_list: &Vec<(String, Vec<String>)>) -> Vec<u8> {
    let mut stream = rlp::RlpStream::new_list(access_list.len());

    for list in access_list {
        let mut stream_tuple = rlp::RlpStream::new_list(2);

        // append address
        stream_tuple.append(&string_to_vec_u8(&list.0[..]));

        // append storage keys
        let mut stream_storage_keys = rlp::RlpStream::new_list(list.1.len());
        for storage_key in list.1.clone() {
            stream_storage_keys.append(&string_to_vec_u8(&storage_key[..]));
        }
        stream_tuple.append_raw(&stream_storage_keys.out(), 1);

        // append (address, storage_keys)
        stream.append_raw(&stream_tuple.out(), 1);
    }

    stream.out().to_vec()
}

fn decode_access_list(access_list: &Vec<u8>) -> Vec<(String, Vec<String>)> {
    let mut decoded_access_list = vec![];
    let rlp = rlp::Rlp::new(access_list);
    for item in rlp.iter() {
        let address = item.at(0).as_val();
        let storage_keys_u8 = item.at(1).as_list::<Vec<u8>>();
        let storage_keys = storage_keys_u8
            .iter()
            .map(|x| vec_u8_to_string(x))
            .collect::<Vec<String>>();
        decoded_access_list.push((vec_u8_to_string(&address), storage_keys));
    }
    decoded_access_list
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_recovery_id_valid() {
        let expected = 0;

        let public_key = string_to_vec_u8("02c397f23149d3464517d57b7cdc8e287428407f9beabfac731e7c24d536266cd1");
        let signature =string_to_vec_u8("29edd4e1d65e1b778b464112d2febc6e97bb677aba5034408fd27b49921beca94c4e5b904d58553bcd9c788360e0bd55c513922cf1f33a6386033e886cd4f77f");
        let message = string_to_vec_u8("79965df63d7d9364f4bc8ed54ffd1c267042d4db673e129e3c459afbcb73a6f1");
        let result = get_recovery_id(&message, &signature, &public_key).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn get_recovery_id_with_invalid_signature() {
        let expected = Err("Invalid signature".to_string());

        let public_key = string_to_vec_u8("02c397f23149d3464517d57b7cdc8e287428407f9beabfac731e7c24d536266cd1");
        let signature = string_to_vec_u8("");
        let message = string_to_vec_u8("79965df63d7d9364f4bc8ed54ffd1c267042d4db673e129e3c459afbcb73a6f1");
        let result = get_recovery_id(&message, &signature, &public_key);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_recovery_id_with_invalid_message() {
        let expected = Err("Invalid message".to_string());

        let public_key = string_to_vec_u8("02c397f23149d3464517d57b7cdc8e287428407f9beabfac731e7c24d536266cd1");
        let signature = string_to_vec_u8("29edd4e1d65e1b778b464112d2febc6e97bb677aba5034408fd27b49921beca94c4e5b904d58553bcd9c788360e0bd55c513922cf1f33a6386033e886cd4f77f");
        let message = string_to_vec_u8("");
        let result = get_recovery_id(&message, &signature, &public_key);
        assert_eq!(result, expected);
    }

    #[test]
    fn get_recovery_id_with_invalid_public_key() {
        let expected = Err("Invalid public key".to_string());

        let public_key = string_to_vec_u8("");
        let signature = string_to_vec_u8("29edd4e1d65e1b778b464112d2febc6e97bb677aba5034408fd27b49921beca94c4e5b904d58553bcd9c788360e0bd55c513922cf1f33a6386033e886cd4f77f");
        let message = string_to_vec_u8("79965df63d7d9364f4bc8ed54ffd1c267042d4db673e129e3c459afbcb73a6f1");
        let result = get_recovery_id(&message, &signature, &public_key);
        assert_eq!(result, expected);
    }

    #[test]
    fn access_list_encode() {
        let expected = "f872f85994de0b295669a9fd93d5f28d9ec85e40f4cb697baef842a00000000000000000000000000000000000000000000000000000000000000003a00000000000000000000000000000000000000000000000000000000000000007d694bb9bc244d798123fde783fcc1c72d3bb8c189413c0";
        let address_1 = "0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae".to_string();
        let storage_keys_1 = vec![
            "0x0000000000000000000000000000000000000000000000000000000000000003".to_string(),
            "0x0000000000000000000000000000000000000000000000000000000000000007".to_string(),
        ];

        let address_2 = "0xbb9bc244d798123fde783fcc1c72d3bb8c189413".to_string();
        let storage_keys_2 = vec![];

        let access_list = vec![(address_1, storage_keys_1), (address_2, storage_keys_2)];
        let encoded = encode_access_list(&access_list);
        assert_eq!(vec_u8_to_string(&encoded), expected)
    }

    #[test]
    fn access_list_decode() {
        let expected: Vec<(String, Vec<String>)> = vec![
            (
                "de0b295669a9fd93d5f28d9ec85e40f4cb697bae".to_string(),
                vec![
                    "0000000000000000000000000000000000000000000000000000000000000003".to_string(),
                    "0000000000000000000000000000000000000000000000000000000000000007".to_string(),
                ],
            ),
            (
                "bb9bc244d798123fde783fcc1c72d3bb8c189413".to_string(),
                vec![],
            ),
        ];
        let access_list = "f872f85994de0b295669a9fd93d5f28d9ec85e40f4cb697baef842a00000000000000000000000000000000000000000000000000000000000000003a00000000000000000000000000000000000000000000000000000000000000007d694bb9bc244d798123fde783fcc1c72d3bb8c189413c0";
        let access_list_hex = string_to_vec_u8(&access_list);

        let decoded = decode_access_list(&access_list_hex);
        assert_eq!(decoded, expected);
    }
}
