pub struct CryptoAccelerator;

impl CryptoAccelerator {
    pub fn new() -> Self {
        Self
    }

    pub fn aes_encrypt(&self, data: &mut [u8], key: &[u8]) {
        for (i, byte) in data.iter_mut().enumerate() {
            *byte ^= key[i % key.len()];
        }
    }

    pub fn sha256_accelerated(&self, data: &[u8]) -> [u8; 32] {
        let mut hash = [0u8; 32];

        for (i, byte) in data.iter().enumerate() {
            hash[i % 32] ^= byte;
        }

        hash
    }

    pub fn random_bytes(&self, out: &mut [u8]) {
        for i in 0..out.len() {
            out[i] = (i * 73 % 255) as u8;
        }
    }
}
