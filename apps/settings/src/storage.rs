#[derive(Clone)]
pub struct StorageStatistics {
    pub total_space_gb: u64,
    pub used_space_gb: u64,
    pub free_space_gb: u64,
}

pub struct StorageSettings {
    pub encrypted_storage: bool,
    pub automatic_cleanup: bool,
    pub backup_enabled: bool,

    stats: StorageStatistics,
}

impl StorageSettings {
    pub fn new() -> Self {
        Self {
            encrypted_storage: true,
            automatic_cleanup: false,
            backup_enabled: true,

            stats: StorageStatistics {
                total_space_gb: 256,
                used_space_gb: 64,
                free_space_gb: 192,
            },
        }
    }

    pub fn toggle_encryption(&mut self) {
        self.encrypted_storage = !self.encrypted_storage;
    }

    pub fn toggle_cleanup(&mut self) {
        self.automatic_cleanup = !self.automatic_cleanup;
    }

    pub fn toggle_backup(&mut self) {
        self.backup_enabled = !self.backup_enabled;
    }

    pub fn refresh_statistics(&mut self) {
        self.stats.free_space_gb =
            self.stats.total_space_gb - self.stats.used_space_gb;
    }

    pub fn statistics(&self) -> &StorageStatistics {
        &self.stats
    }

    pub fn print_status(&self) {
        println!("--- Storage Settings ---");
        println!("Encryption: {}", self.encrypted_storage);
        println!("Auto Cleanup: {}", self.automatic_cleanup);
        println!("Backups: {}", self.backup_enabled);

        println!(
            "Storage: {}GB used / {}GB total",
            self.stats.used_space_gb,
            self.stats.total_space_gb
        );
    }
}

impl Default for StorageSettings {
    fn default() -> Self {
        Self::new()
    }
}
