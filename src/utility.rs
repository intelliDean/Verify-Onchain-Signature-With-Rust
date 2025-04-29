use ethers::prelude::{Bytes, Signature};

// Convert Signature to Bytes
pub fn to_bytes(signature: Signature) -> Bytes {
    Bytes::from(signature.to_vec())
}
