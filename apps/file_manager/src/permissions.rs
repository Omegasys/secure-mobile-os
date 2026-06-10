use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone)]
pub struct FilePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub delete: bool,
}

pub struct PermissionManager {
    permissions:
        HashMap<PathBuf, FilePermissions>,
}

impl PermissionManager {
    pub fn new() -> Self {
        Self {
            permissions: HashMap::new(),
        }
    }

    pub fn set_permissions(
        &mut self,
        path: PathBuf,
        perms: FilePermissions,
    ) {
        self.permissions.insert(
            path,
            perms,
        );
    }

    pub fn get_permissions(
        &self,
        path: &PathBuf,
    ) -> Option<&FilePermissions> {
        self.permissions.get(path)
    }

    pub fn can_read(
        &self,
        path: &PathBuf,
    ) -> bool {
        self.permissions
            .get(path)
            .map(|p| p.read)
            .unwrap_or(false)
    }

    pub fn can_write(
        &self,
        path: &PathBuf,
    ) -> bool {
        self.permissions
            .get(path)
            .map(|p| p.write)
            .unwrap_or(false)
    }

    pub fn can_execute(
        &self,
        path: &PathBuf,
    ) -> bool {
        self.permissions
            .get(path)
            .map(|p| p.execute)
            .unwrap_or(false)
    }

    pub fn can_delete(
        &self,
        path: &PathBuf,
    ) -> bool {
        self.permissions
            .get(path)
            .map(|p| p.delete)
            .unwrap_or(false)
    }
}
