use rand_core::{CryptoRng, Error, RngCore};

pub struct FakeCryptoRng {
    value: u8
}

impl FakeCryptoRng {
    pub fn new(return_value: u8) -> Self {
        Self {
            value: return_value
        }
    }
}

impl RngCore for FakeCryptoRng {
    fn next_u32(&mut self) -> u32 {
        self.value as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.value as u64
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.fill(self.value)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        dest.fill(self.value);
        Ok(())
    }
}

impl CryptoRng for FakeCryptoRng {}