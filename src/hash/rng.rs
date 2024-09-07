use scrypt::password_hash::rand_core;

#[derive(Clone)]
pub struct DefaultRng;

impl rand_core::CryptoRng for DefaultRng {}

impl rand_core::RngCore for DefaultRng {
    fn next_u32(&mut self) -> u32 {
        rand_core::OsRng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        rand_core::OsRng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}
