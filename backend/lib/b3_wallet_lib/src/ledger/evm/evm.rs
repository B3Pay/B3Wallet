use crate::ledger::error::LedgerError;
use crate::ledger::ledger::Ledger;
use crate::ledger::subaccount::SubaccountTrait;

use super::berlin::EvmTransaction2930;
use super::error::EvmError;
use super::legacy::EvmTransactionLegacy;
use super::london::EvmTransaction1559;
use super::utils::{string_to_vec_u8, vec_u8_to_string};
use bitcoin::secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Message, PublicKey, Secp256k1,
};
use candid::{CandidType, Deserialize};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait EvmSignTrait {
    fn sign(&mut self, signature: Vec<u8>, public_key: PublicKey) -> Result<Vec<u8>, EvmError>;
    fn signature(&self) -> Result<Vec<u8>, EvmError>;
    fn recovery_id(&self) -> Result<u8, EvmError>;
    fn unsigned_serialized(&self) -> Vec<u8>;
    fn serialized(&self) -> Vec<u8>;
    fn is_signed(&self) -> bool;
    fn nonce(&self) -> u64;
    fn chain_id(&self) -> u64;
    fn hash(&self) -> Vec<u8>;
    fn unsigned_hash(&self) -> Vec<u8>;
    fn tx_id(&self) -> String;
}

#[enum_dispatch(EvmSignTrait)]
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum EvmTransaction {
    EvmTransactionLegacy,
    EvmTransaction1559,
    EvmTransaction2930,
}

#[derive(Clone, Deserialize, PartialEq, CandidType, Debug)]
pub enum EvmTransactionType {
    Legacy,
    EIP1559,
    EIP2930,
}

impl EvmTransactionType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            EvmTransactionType::Legacy => "legacy",
            EvmTransactionType::EIP1559 => "eip1559",
            EvmTransactionType::EIP2930 => "eip2930",
        }
    }
}

pub fn get_evm_transaction(
    hex_raw_tx: &Vec<u8>,
    chain_id: u64,
) -> Result<EvmTransaction, EvmError> {
    let tx_type = get_evm_transaction_type(hex_raw_tx)?;

    if tx_type == EvmTransactionType::Legacy {
        let transaction = EvmTransactionLegacy::from((hex_raw_tx.to_owned(), chain_id));

        Ok(transaction.into())
    } else if tx_type == EvmTransactionType::EIP1559 {
        let transaction = EvmTransaction1559::from(hex_raw_tx.to_owned());

        Ok(transaction.into())
    } else if tx_type == EvmTransactionType::EIP2930 {
        let transaction = EvmTransaction2930::from(hex_raw_tx.to_owned());

        Ok(transaction.into())
    } else {
        Err(EvmError::InvalidTransactionType)
    }
}

pub fn get_evm_transaction_type(hex_raw_tx: &Vec<u8>) -> Result<EvmTransactionType, EvmError> {
    if hex_raw_tx[0] >= 0xc0 {
        Ok(EvmTransactionType::Legacy)
    } else if hex_raw_tx[0] == 0x01 {
        Ok(EvmTransactionType::EIP2930)
    } else if hex_raw_tx[0] == 0x02 {
        Ok(EvmTransactionType::EIP1559)
    } else {
        Err(EvmError::InvalidTransactionType)
    }
}

pub fn get_recovery_id(
    message: &[u8],
    signature: &[u8],
    public_key: &PublicKey,
) -> Result<u8, EvmError> {
    let message =
        Message::from_slice(message).map_err(|err| EvmError::InvalidMessage(err.to_string()))?;

    let secp = Secp256k1::verification_only();

    for i in 0..4 {
        let recovery_id =
            RecoveryId::from_i32(i).map_err(|err| EvmError::InvalidRecoveryId(err.to_string()))?;

        let signature = RecoverableSignature::from_compact(signature, recovery_id)
            .map_err(|err| EvmError::InvalidSignature(err.to_string()))?;

        let recovered_key = secp
            .recover_ecdsa(&message, &signature)
            .map_err(|err| EvmError::InvalidSignature(err.to_string()))?;

        if recovered_key.eq(&public_key) {
            return Ok(i as u8);
        }
    }

    Err(EvmError::InvalidSignature(
        "Not able to recover public key".to_string(),
    ))
}

pub fn encode_access_list(access_list: &Vec<(String, Vec<String>)>) -> Vec<u8> {
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

pub fn decode_access_list(access_list: &Vec<u8>) -> Vec<(String, Vec<String>)> {
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

impl Ledger {
    pub async fn sign_evm_transaction(
        &self,
        hex_raw_tx: Vec<u8>,
        chain_id: u64,
    ) -> Result<Vec<u8>, LedgerError> {
        let public_key = self.eth_public_key()?;

        let mut evm_tx =
            get_evm_transaction(&hex_raw_tx, chain_id).map_err(LedgerError::EvmError)?;

        let message = evm_tx.unsigned_serialized();

        if message.len() != 32 {
            return Err(LedgerError::InvalidMessageLength);
        }

        let signature = self.subaccount.sign_with_ecdsa(message).await?;

        let signed_evm_tx = evm_tx
            .sign(signature, public_key)
            .map_err(LedgerError::EvmError)?;

        Ok(signed_evm_tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_recovery_id_valid() {
        let expected = 0;

        let pub_key =
            string_to_vec_u8("02c397f23149d3464517d57b7cdc8e287428407f9beabfac731e7c24d536266cd1");

        let public_key = PublicKey::from_slice(&pub_key).unwrap();

        let signature =string_to_vec_u8("29edd4e1d65e1b778b464112d2febc6e97bb677aba5034408fd27b49921beca94c4e5b904d58553bcd9c788360e0bd55c513922cf1f33a6386033e886cd4f77f");
        let message =
            string_to_vec_u8("79965df63d7d9364f4bc8ed54ffd1c267042d4db673e129e3c459afbcb73a6f1");
        let result = get_recovery_id(&message, &signature, &public_key).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn get_recovery_id_with_invalid_signature() {
        let expected = Err(EvmError::InvalidSignature(
            "malformed signature".to_string(),
        ));

        let pub_key =
            string_to_vec_u8("02c397f23149d3464517d57b7cdc8e287428407f9beabfac731e7c24d536266cd1");

        let public_key = PublicKey::from_slice(&pub_key).unwrap();

        let signature = string_to_vec_u8("");
        let message =
            string_to_vec_u8("79965df63d7d9364f4bc8ed54ffd1c267042d4db673e129e3c459afbcb73a6f1");
        let result = get_recovery_id(&message, &signature, &public_key);

        assert_eq!(result, expected);
    }

    #[test]
    fn get_recovery_id_with_invalid_message() {
        let expected = Err(EvmError::InvalidMessage(
            "message was not 32 bytes (do you need to hash?)".to_string(),
        ));

        let pub_key =
            string_to_vec_u8("02c397f23149d3464517d57b7cdc8e287428407f9beabfac731e7c24d536266cd1");

        let public_key = PublicKey::from_slice(&pub_key).unwrap();

        let signature = string_to_vec_u8("29edd4e1d65e1b778b464112d2febc6e97bb677aba5034408fd27b49921beca94c4e5b904d58553bcd9c788360e0bd55c513922cf1f33a6386033e886cd4f77f");
        let message = string_to_vec_u8("");
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
