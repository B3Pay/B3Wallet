use super::error::EvmError;
use super::evm::{decode_access_list, encode_access_list, get_recovery_id, EvmSignTrait};
use super::utils::{
    remove_leading, string_to_vec_u8, u64_to_vec_u8, vec_u8_to_string, vec_u8_to_u64,
};
use b3_helper_lib::raw_keccak256;
use bitcoin::secp256k1::PublicKey;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmTransaction1559 {
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

impl From<Vec<u8>> for EvmTransaction1559 {
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

        EvmTransaction1559 {
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
impl EvmSignTrait for EvmTransaction1559 {
    fn sign(&mut self, signature: Vec<u8>, public_key: PublicKey) -> Result<Vec<u8>, EvmError> {
        let r_remove_leading_zeros = remove_leading(signature[..32].to_vec(), 0);
        let s_remove_leading_zeros = remove_leading(signature[32..].to_vec(), 0);

        let r = vec_u8_to_string(&r_remove_leading_zeros);

        let s = vec_u8_to_string(&s_remove_leading_zeros);

        let message = self.unsigned_serialized();

        let recovery_id = get_recovery_id(&message, &signature, &public_key)?;

        let v: String;
        if recovery_id == 0 {
            v = "".to_string();
        } else {
            v = "01".to_string();
        }

        self.v = v;
        self.r = r;
        self.s = s;

        let result = self.serialized();

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

    fn signature(&self) -> Result<Vec<u8>, EvmError> {
        if !self.is_signed() {
            return Err(EvmError::NotSignedTransaction);
        }

        let r = string_to_vec_u8(&self.r);
        let s = string_to_vec_u8(&self.s);

        Ok([&r[..], &s[..]].concat())
    }

    fn recovery_id(&self) -> Result<u8, EvmError> {
        if !self.is_signed() {
            return Err(EvmError::NotSignedTransaction);
        }

        let v = &self.v;

        if v.is_empty() {
            Ok(0 as u8)
        } else {
            Ok(1 as u8)
        }
    }

    fn unsigned_serialized(&self) -> Vec<u8> {
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

        for item in items {
            stream.append(&item);
        }

        let access_list = encode_access_list(&self.access_list);
        stream.append_raw(&access_list, 1);

        let decode_tx = stream.out();
        let encoded_tx = [&[0x02], &decode_tx[..]].concat();

        encoded_tx
    }

    fn serialized(&self) -> Vec<u8> {
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

        let serialized = [&[0x02], &result[..]].concat();

        serialized
    }

    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn chain_id(&self) -> u64 {
        self.chain_id
    }

    fn hash(&self) -> Vec<u8> {
        let result = self.serialized();
        let hash = raw_keccak256(&result);

        hash.to_vec()
    }

    fn unsigned_hash(&self) -> Vec<u8> {
        let result = self.unsigned_serialized();
        let hash = raw_keccak256(&result);

        hash.to_vec()
    }

    fn tx_id(&self) -> String {
        let hash = self.hash();
        let tx_id = hex::encode(hash);

        tx_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_serialization() {
        // create a mock transaction
        let tx = EvmTransaction1559 {
            chain_id: 1,
            nonce: 0,
            max_priority_fee_per_gas: 0,
            max_fee_per_gas: 0,
            gas_limit: 0,
            to: "0x".to_string(),
            value: 0,
            data: "0x".to_string(),
            access_list: vec![],
            v: "".to_string(),
            r: "".to_string(),
            s: "".to_string(),
        };

        // compute the unsigned serialization
        let serialized = tx.unsigned_serialized();

        let expected = [2, 201, 1, 128, 128, 128, 128, 128, 128, 128, 192];
        // print or check the serialized data
        println!("{:?}", serialized);

        assert_eq!(serialized, expected);

        // compute the unsigned serialization
        let serialized = tx.serialized();

        let expected = [
            2, 204, 1, 128, 128, 128, 128, 128, 128, 128, 192, 128, 128, 128,
        ];

        // print or check the serialized data
        println!("{:?}", serialized);

        assert_eq!(serialized, expected);

        // compute the unsigned hash
        let hash = tx.unsigned_hash();

        let expected = [
            133, 242, 59, 76, 29, 61, 57, 187, 85, 31, 18, 132, 123, 246, 18, 51, 25, 237, 88, 76,
            191, 249, 189, 252, 62, 129, 64, 145, 146, 214, 158, 223,
        ];

        // print or check the hash
        println!("{:?}", hash);

        assert_eq!(hash, expected);
    }
}
