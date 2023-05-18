use super::{get_recovery_id, EvmSign, EvmTransaction, EvmTransactionType};
use crate::{
    error::WalletError,
    utils::{remove_leading, string_to_vec_u8, u64_to_vec_u8, vec_u8_to_string, vec_u8_to_u64},
};
use easy_hasher::easy_hasher;

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

impl From<&EvmTransactionLegacy> for EvmTransaction {
    fn from(tx: &EvmTransactionLegacy) -> Self {
        EvmTransaction {
            chain_id: tx.chain_id,
            nonce: tx.nonce,
            gas_price: Some(tx.gas_price),
            gas_limit: tx.gas_limit,
            max_priority_fee_per_gas: None,
            max_fee_per_gas: None,
            to: tx.to.clone(),
            value: tx.value,
            data: tx.data.clone(),
            access_list: None,
            v: tx.v.clone(),
            r: tx.r.clone(),
            s: tx.s.clone(),
            transaction_type: EvmTransactionType::Legacy,
        }
    }
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
impl EvmSign for EvmTransactionLegacy {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, WalletError> {
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
    fn sign(&mut self, signature: Vec<u8>, public_key: Vec<u8>) -> Result<Vec<u8>, WalletError> {
        let chain_id = u64::try_from(self.chain_id).unwrap();

        let r_remove_leading_zeros = remove_leading(signature[..32].to_vec(), 0);
        let s_remove_leading_zeros = remove_leading(signature[32..].to_vec(), 0);

        let r = vec_u8_to_string(&r_remove_leading_zeros);

        let s = vec_u8_to_string(&s_remove_leading_zeros);

        let message = self.get_message_to_sign()?;
        let recovery_id = get_recovery_id(&message, &signature, &public_key)?;

        let v_number = chain_id * 2 + 35 + u64::try_from(recovery_id).unwrap();
        let v = vec_u8_to_string(&u64_to_vec_u8(&v_number));

        self.v = v;
        self.r = r;
        self.s = s;

        let result = self.serialize()?;
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
    fn get_signature(&self) -> Result<Vec<u8>, WalletError> {
        if !self.is_signed() {
            return Err(WalletError::NotSignedTransaction);
        }

        let r = string_to_vec_u8(&self.r);
        let s = string_to_vec_u8(&self.s);

        Ok([&r[..], &s[..]].concat())
    }
    fn get_recovery_id(&self) -> Result<u8, WalletError> {
        if !self.is_signed() {
            return Err(WalletError::NotSignedTransaction);
        }
        let chain_id = i64::try_from(self.chain_id).unwrap();
        let v = string_to_vec_u8(&self.v);

        let recovery_id = -1 * ((chain_id * 2) + 35 - i64::try_from(v[0]).unwrap());
        Ok(u8::try_from(recovery_id).unwrap())
    }
    fn serialize(&self) -> Result<Vec<u8>, WalletError> {
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
    fn get_nonce(&self) -> Result<u64, WalletError> {
        Ok(self.nonce)
    }

    fn get_transaction(&self) -> EvmTransaction {
        self.into()
    }
}
