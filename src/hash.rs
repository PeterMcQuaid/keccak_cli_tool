use primitive_types::H256;

pub fn return_hash(digest: &mut [u8]) -> H256 {
    keccak_hash::keccak(digest)
}
