use std::ops::Shl;

use potatoe::scrypt::{OsRng, RngCore};

pub trait IdGenerator {
    fn new_u64(&self) -> u64;
    fn new_u128(&self) -> u128;
}

pub struct DefaultIdGenerator;

impl IdGenerator for DefaultIdGenerator {
    fn new_u128(&self) -> u128 {
        let sm = OsRng.next_u64() as u128;
        let lg: u128 = sm.shl(64);
        return lg + (OsRng.next_u64() as u128);
    }
    fn new_u64(&self) -> u64 {
        OsRng.next_u64()
    }
}

impl DefaultIdGenerator {
    pub fn init() -> Self {
        return Self;
    }
}
