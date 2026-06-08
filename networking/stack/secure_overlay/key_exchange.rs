pub const PUBLIC_KEY_SIZE: usize = 32;

pub struct PublicKey {
    pub bytes: [u8; PUBLIC_KEY_SIZE],
}

pub struct SharedSecret {
    pub bytes: [u8; 32],
}

pub struct KeyExchange;

impl KeyExchange {
    pub fn establish(
        local: &PublicKey,
        remote: &PublicKey,
    ) -> SharedSecret {
        let mut secret = [0u8; 32];

        for i in 0..32 {
            secret[i] = local.bytes[i] ^ remote.bytes[i];
        }

        SharedSecret {
            bytes: secret,
        }
    }
}
