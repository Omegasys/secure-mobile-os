pub struct BackupEngine;

impl BackupEngine {
    pub fn create_backup(data: &[u8]) -> Vec<u8> {
        // placeholder compression + copy
        data.to_vec()
    }

    pub fn restore_backup(backup: &[u8]) -> Vec<u8> {
        backup.to_vec()
    }
}
