use b3_helper_lib::sha3_sha256;

use super::utils::{
    remove_leading, string_to_vec_u8, u64_to_vec_u8, vec_u8_to_string, vec_u8_to_u64,
};
use super::{
    decode_access_list, encode_access_list, get_recovery_id, EvmSign, EvmTransaction,
    EvmTransactionType,
};
use crate::error::WalletError;

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

impl From<&EvmTransaction1559> for EvmTransaction {
    fn from(tx: &EvmTransaction1559) -> Self {
        EvmTransaction {
            chain_id: tx.chain_id,
            nonce: tx.nonce,
            gas_price: None,
            gas_limit: tx.gas_limit,
            max_fee_per_gas: Some(tx.max_fee_per_gas),
            max_priority_fee_per_gas: Some(tx.max_priority_fee_per_gas),
            to: tx.to.clone(),
            value: tx.value,
            data: tx.data.clone(),
            access_list: None,
            v: tx.v.clone(),
            r: tx.r.clone(),
            s: tx.s.clone(),
            transaction_type: EvmTransactionType::EIP1559,
        }
    }
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
impl EvmSign for EvmTransaction1559 {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, WalletError> {
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

        let encoded_tx = [&[0x02], &decode_tx[..]].concat();

        let result = sha3_sha256(&encoded_tx);

        Ok(result)
    }
    fn sign(&mut self, signature: Vec<u8>, public_key: Vec<u8>) -> Result<Vec<u8>, WalletError> {
        let r_remove_leading_zeros = remove_leading(signature[..32].to_vec(), 0);
        let s_remove_leading_zeros = remove_leading(signature[32..].to_vec(), 0);

        let r = vec_u8_to_string(&r_remove_leading_zeros);

        let s = vec_u8_to_string(&s_remove_leading_zeros);

        let message = self.get_message_to_sign()?;
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
        let v = &self.v;

        if v.is_empty() {
            Ok(0 as u8)
        } else {
            Ok(1 as u8)
        }
    }
    fn serialize(&self) -> Result<Vec<u8>, WalletError> {
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
    fn get_nonce(&self) -> Result<u64, WalletError> {
        Ok(self.nonce)
    }
    fn get_transaction(&self) -> EvmTransaction {
        self.into()
    }
}
