#[derive(Clone, Copy)]
pub enum Permission {
    Read,
    Write,
    Execute,
}

pub struct FilePermission {
    pub owner_uid: u64,
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl FilePermission {
    pub fn can_read(&self, uid: u64) -> bool {
        uid == self.owner_uid && self.read
    }

    pub fn can_write(&self, uid: u64) -> bool {
        uid == self.owner_uid && self.write
    }
}
