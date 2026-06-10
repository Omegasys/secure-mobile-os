use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone)]
pub struct EncryptionMetadata {
    pub encrypted: bool,
    pub algorithm: String,
    pub key_id: String,
}

pub struct EncryptionManager {
    files:
        HashMap<PathBuf, EncryptionMetadata>,
}

impl EncryptionManager {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn register_file(
        &mut self,
        path: PathBuf,
        metadata: EncryptionMetadata,
    ) {
        self.files.insert(path, metadata);
    }

    pub fn is_encrypted(
        &self,
        path: &PathBuf,
    ) -> bool {
        self.files
            .get(path)
            .map(|m| m.encrypted)
            .unwrap_or(false)
    }

    pub fn metadata(
        &self,
        path: &PathBuf,
    ) -> Option<&EncryptionMetadata> {
        self.files.get(path)
    }
}
