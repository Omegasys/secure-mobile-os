use std::collections::HashMap;

#[derive(Clone)]
pub struct AppPermissions {
    pub camera: bool,
    pub microphone: bool,
    pub location: bool,
    pub storage: bool,
    pub contacts: bool,
    pub network: bool,
}

pub struct PermissionDashboard {
    permissions:
        HashMap<String, AppPermissions>,
}

impl PermissionDashboard {
    pub fn new() -> Self {
        Self {
            permissions: HashMap::new(),
        }
    }

    pub fn register_app(
        &mut self,
        app_name: String,
        permissions: AppPermissions,
    ) {
        self.permissions.insert(
            app_name,
            permissions,
        );
    }

    pub fn remove_app(
        &mut self,
        app_name: &str,
    ) {
        self.permissions.remove(app_name);
    }

    pub fn app_count(&self) -> usize {
        self.permissions.len()
    }

    pub fn permissions_for(
        &self,
        app_name: &str,
    ) -> Option<&AppPermissions> {
        self.permissions.get(app_name)
    }

    pub fn applications(
        &self,
    ) -> Vec<&String> {
        self.permissions.keys().collect()
    }
}

impl Default for PermissionDashboard {
    fn default() -> Self {
        Self::new()
    }
}
