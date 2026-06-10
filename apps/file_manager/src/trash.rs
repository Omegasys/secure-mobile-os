use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct TrashEntry {
    pub id: u64,
    pub original_path: PathBuf,
    pub deleted_at: u64,
}

pub struct TrashManager {
    entries: Vec<TrashEntry>,
}

impl TrashManager {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn move_to_trash(
        &mut self,
        path: PathBuf,
    ) {
        self.entries.push(TrashEntry {
            id: timestamp(),
            original_path: path,
            deleted_at: timestamp(),
        });
    }

    pub fn restore(
        &mut self,
        id: u64,
    ) -> Option<PathBuf> {
        let position = self
            .entries
            .iter()
            .position(|e| e.id == id)?;

        Some(
            self.entries
                .remove(position)
                .original_path,
        )
    }

    pub fn empty_trash(&mut self) {
        self.entries.clear();
    }

    pub fn entries(
        &self,
    ) -> &[TrashEntry] {
        &self.entries
    }
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
