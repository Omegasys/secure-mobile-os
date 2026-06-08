pub const SESSION_KEY_SIZE: usize = 32;

pub struct SessionKey {
    pub bytes: [u8; SESSION_KEY_SIZE],
}

pub struct EncryptionEngine;

impl EncryptionEngine {
    pub fn encrypt(
        data: &mut [u8],
        key: &SessionKey,
    ) {
        for (i, byte) in data.iter_mut().enumerate() {
            *byte ^= key.bytes[i % SESSION_KEY_SIZE];
        }
    }

    pub fn decrypt(
        data: &mut [u8],
        key: &SessionKey,
    ) {
        Self::encrypt(data, key);
    }
}
