use ic_bls12_381::{
    hash_to_curve::{ExpandMsgXmd, HashToCurve},
    pairing, G1Affine, G1Projective, G2Affine, G2Prepared, G2Projective, Gt,
};

use self::error::PairingError;

mod error;

const G1AFFINE_BYTES: usize = 48; // Size of compressed form
const G2AFFINE_BYTES: usize = 96; // Size of compressed form

pub fn gt_multipairing(terms: &[(&G1Affine, &G2Prepared)]) -> Gt {
    ic_bls12_381::multi_miller_loop(terms).final_exponentiation()
}

pub fn option_from_ctoption<T>(ctoption: subtle::CtOption<T>) -> Option<T> {
    if bool::from(ctoption.is_some()) {
        Some(ctoption.unwrap())
    } else {
        None
    }
}

pub fn augmented_hash_to_g1(pk: &G2Affine, data: &[u8]) -> G1Affine {
    let domain_sep = b"BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_AUG_";

    let mut signature_input = Vec::with_capacity(G2AFFINE_BYTES + data.len());
    signature_input.extend_from_slice(&pk.to_compressed());
    signature_input.extend_from_slice(data);

    let pt = <G1Projective as HashToCurve<ExpandMsgXmd<sha2::Sha256>>>::hash_to_curve(
        signature_input,
        domain_sep,
    );
    G1Affine::from(pt)
}

pub fn augmented_hash_to_g2(pk: &G1Affine, data: &[u8]) -> G2Affine {
    let domain_sep = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_AUG_"; // Notice G2 in domain separator

    let mut signature_input = Vec::with_capacity(G1AFFINE_BYTES + data.len());
    signature_input.extend_from_slice(&pk.to_compressed());
    signature_input.extend_from_slice(data);

    let pt = <G2Projective as HashToCurve<ExpandMsgXmd<sha2::Sha256>>>::hash_to_curve(
        signature_input,
        domain_sep,
    );
    G2Affine::from(pt)
}

pub fn deserialize_g1(bytes: &[u8]) -> Result<G1Affine, PairingError> {
    let bytes: &[u8; G1AFFINE_BYTES] = bytes.try_into().map_err(|_| PairingError::InvalidLength)?;

    let pt = G1Affine::from_compressed(bytes);
    if bool::from(pt.is_some()) {
        Ok(pt.unwrap())
    } else {
        Err(PairingError::InvalidCurve)
    }
}

pub fn deserialize_g2(bytes: &[u8]) -> Result<G2Affine, PairingError> {
    let bytes: &[u8; G2AFFINE_BYTES] = bytes.try_into().map_err(|_| PairingError::InvalidLength)?;

    let pt = G2Affine::from_compressed(bytes);
    if bool::from(pt.is_some()) {
        Ok(pt.unwrap())
    } else {
        Err(PairingError::InvalidCurve)
    }
}

pub fn verify_pairing(
    public_key: &[u8],
    signature: &[u8],
    input: &[u8],
) -> Result<bool, PairingError> {
    // Deserialize the public key from G2 and signature from G1
    let public_key = deserialize_g1(public_key)?;
    let signature = deserialize_g2(signature)?;

    // Hash the input to a point on G2 using the public key in G1
    let hashed_input = augmented_hash_to_g2(&G1Affine::generator(), input);
    // Pairings
    let pairing_check_1 = pairing(&G1Affine::generator(), &signature);
    let pairing_check_2 = pairing(&public_key, &hashed_input);

    // Verify if the pairings are equal
    Ok(pairing_check_1 == pairing_check_2)
}
