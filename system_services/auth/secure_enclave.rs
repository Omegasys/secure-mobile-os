pub struct SecureEnclave {
    key_slot: [u8; 32],
}

impl SecureEnclave {
    pub const fn new() -> Self {
        Self {
            key_slot: [0; 32],
        }
    }

    pub fn store_key(&mut self, key: [u8; 32]) {
        self.key_slot = key;
    }

    pub fn get_key(&self) -> [u8; 32] {
        self.key_slot
    }

    pub fn sign(&self, data: &[u8]) -> [u8; 32] {
        let mut out = [0u8; 32];

        for (i, b) in data.iter().enumerate().take(32) {
            out[i] = b ^ self.key_slot[i];
        }

        out
    }
}
