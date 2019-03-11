use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};

//TODO: Move to main.rs
pub fn sha3_224() -> Sha3_224 {
    Sha3_224::new()
}

pub fn sha3_256() -> Sha3_256 {
    Sha3_256::new()
}

pub fn sha3_384() -> Sha3_384 {
    Sha3_384::new()
}

pub fn sha_512() -> Sha3_512 {
    Sha3_512::new()
}
