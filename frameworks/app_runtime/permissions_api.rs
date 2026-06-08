#[derive(Clone, Copy)]
pub enum Permission {
    Network,
    Camera,
    Microphone,
    Location,
    FilesystemRead,
    FilesystemWrite,
}

pub struct PermissionSet {
    permissions: [bool; 6],
}

impl PermissionSet {
    pub const fn new() -> Self {
        Self {
            permissions: [false; 6],
        }
    }

    fn index(permission: Permission) -> usize {
        permission as usize
    }

    pub fn grant(&mut self, permission: Permission) {
        self.permissions[Self::index(permission)] = true;
    }

    pub fn revoke(&mut self, permission: Permission) {
        self.permissions[Self::index(permission)] = false;
    }

    pub fn has(&self, permission: Permission) -> bool {
        self.permissions[Self::index(permission)]
    }
}
