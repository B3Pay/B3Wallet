use super::error::EvmError;
use super::evm::{get_recovery_id, EvmSignTrait};
use super::utils::{
    remove_leading, string_to_vec_u8, u64_to_vec_u8, vec_u8_to_string, vec_u8_to_u64,
};
use b3_utils::raw_keccak256;
use bitcoin::secp256k1::PublicKey;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub struct EvmTransactionLegacy {
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

impl From<(Vec<u8>, u64)> for EvmTransactionLegacy {
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

        let chain_id = data.1;

        EvmTransactionLegacy {
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
impl EvmSignTrait for EvmTransactionLegacy {
    fn sign(&mut self, signature: Vec<u8>, public_key: PublicKey) -> Result<Vec<u8>, EvmError> {
        let chain_id = u64::try_from(self.chain_id).unwrap();

        let r_remove_leading_zeros = remove_leading(signature[..32].to_vec(), 0);
        let s_remove_leading_zeros = remove_leading(signature[32..].to_vec(), 0);

        let r = vec_u8_to_string(&r_remove_leading_zeros);

        let s = vec_u8_to_string(&s_remove_leading_zeros);

        let message = self.unsigned_serialized();
        let recovery_id = get_recovery_id(&message, &signature, &public_key)?;

        let v_number = chain_id * 2 + 35 + u64::try_from(recovery_id).unwrap();
        let v = vec_u8_to_string(&u64_to_vec_u8(&v_number));

        self.v = v;
        self.r = r;
        self.s = s;

        let serialized = self.serialized();

        Ok(serialized)
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
        let chain_id = i64::try_from(self.chain_id).unwrap();
        let v = string_to_vec_u8(&self.v);

        let recovery_id = -1 * ((chain_id * 2) + 35 - i64::try_from(v[0]).unwrap());
        Ok(u8::try_from(recovery_id).unwrap())
    }

    fn unsigned_serialized(&self) -> Vec<u8> {
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

        encoded_tx
    }

    fn serialized(&self) -> Vec<u8> {
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

        let serialized = stream.out().to_vec();

        serialized
    }

    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn chain_id(&self) -> u64 {
        self.chain_id
    }

    fn hash(&self) -> Vec<u8> {
        let serialized = self.serialized();
        let result = raw_keccak256(&serialized);

        result.to_vec()
    }

    fn unsigned_hash(&self) -> Vec<u8> {
        let serialized = self.unsigned_serialized();
        let result = raw_keccak256(&serialized);

        result.to_vec()
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
        let tx = EvmTransactionLegacy {
            chain_id: 1,
            nonce: 0,
            gas_limit: 0,
            to: "0x".to_string(),
            value: 0,
            data: "0x".to_string(),
            v: "".to_string(),
            r: "".to_string(),
            s: "".to_string(),
            gas_price: 0,
        };

        // compute the unsigned serialization
        let serialized = tx.unsigned_serialized();

        let expected = [201, 128, 128, 128, 128, 128, 128, 1, 128, 128];

        // print or check the serialized data
        println!("{:?}", serialized);

        assert_eq!(serialized, expected);

        // compute the unsigned serialization
        let serialized = tx.serialized();

        let expected = [201, 128, 128, 128, 128, 128, 128, 128, 128, 128];

        // print or check the serialized data
        println!("{:?}", serialized);

        assert_eq!(serialized, expected);

        // compute the unsigned hash
        let hash = tx.unsigned_hash();

        let expected = [
            15, 205, 131, 60, 62, 124, 213, 227, 30, 241, 78, 221, 11, 123, 130, 248, 92, 146, 203,
            212, 91, 94, 36, 231, 94, 198, 242, 134, 202, 162, 193, 170,
        ];
        // print or check the hash
        println!("{:?}", hash);

        assert_eq!(hash, expected);
    }
}
