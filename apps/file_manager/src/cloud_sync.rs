use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone)]
pub enum SyncState {
    Synced,
    PendingUpload,
    PendingDownload,
    Conflict,
    Error,
}

#[derive(Clone)]
pub struct SyncFile {
    pub path: PathBuf,
    pub state: SyncState,
    pub version: u64,
}

pub struct CloudProvider {
    pub name: String,
    pub endpoint: String,
}

pub struct CloudSyncManager {
    provider: Option<CloudProvider>,
    tracked_files: HashMap<PathBuf, SyncFile>,
    sync_enabled: bool,
}

impl CloudSyncManager {
    pub fn new() -> Self {
        Self {
            provider: None,
            tracked_files: HashMap::new(),
            sync_enabled: false,
        }
    }

    pub fn configure_provider(
        &mut self,
        provider: CloudProvider,
    ) {
        self.provider = Some(provider);
    }

    pub fn enable_sync(&mut self) {
        self.sync_enabled = true;
    }

    pub fn disable_sync(&mut self) {
        self.sync_enabled = false;
    }

    pub fn register_file(
        &mut self,
        path: PathBuf,
    ) {
        self.tracked_files.insert(
            path.clone(),
            SyncFile {
                path,
                state: SyncState::PendingUpload,
                version: 1,
            },
        );
    }

    pub fn mark_synced(
        &mut self,
        path: &PathBuf,
    ) {
        if let Some(file) =
            self.tracked_files.get_mut(path)
        {
            file.state = SyncState::Synced;
        }
    }

    pub fn mark_conflict(
        &mut self,
        path: &PathBuf,
    ) {
        if let Some(file) =
            self.tracked_files.get_mut(path)
        {
            file.state = SyncState::Conflict;
        }
    }

    pub fn sync_all(&self) {
        if !self.sync_enabled {
            println!("Cloud sync disabled");
            return;
        }

        println!("Starting cloud sync...");
    }

    pub fn tracked_files(
        &self,
    ) -> Vec<&SyncFile> {
        self.tracked_files.values().collect()
    }
}

impl Default for CloudSyncManager {
    fn default() -> Self {
        Self::new()
    }
}
