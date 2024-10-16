use rand_core::{CryptoRng, Error, RngCore};

pub struct FakeCryptoRng {}

impl FakeCryptoRng {
    pub fn new() -> Self {
        Self {}
    }
}

impl RngCore for FakeCryptoRng {
    fn next_u32(&mut self) -> u32 {
        0
    }

    fn next_u64(&mut self) -> u64 {
        0
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.fill(0)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        dest.fill(0);
        Ok(())
    }
}

impl CryptoRng for FakeCryptoRng {}