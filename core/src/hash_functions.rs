//! all the hash functions used in LOKI
use sha3::*;
use sm3::{Digest, Sm3};

/// the implementation of keccak256 hash function
pub fn keccak256(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize()[..].to_vec()
}

/// the implementation of sha3 hash function
pub fn sha3_256(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize()[..].to_vec()
}

/// the implementation of sm3 function
pub fn sm3(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sm3::new();
    hasher.update(data);
    hasher.finalize()[..].to_vec()
}
