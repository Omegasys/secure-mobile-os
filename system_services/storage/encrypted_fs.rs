pub struct EncryptedFile {
    pub path: &'static str,
    pub data: Vec<u8>,
}

pub struct EncryptedFS;

impl EncryptedFS {
    pub fn encrypt(data: &mut [u8], key: u8) {
        for byte in data {
            *byte ^= key;
        }
    }

    pub fn decrypt(data: &mut [u8], key: u8) {
        Self::encrypt(data, key);
    }
}
