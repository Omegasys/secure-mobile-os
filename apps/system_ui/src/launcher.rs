use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Application {
    pub id: u64,
    pub name: String,
    pub package_name: String,
    pub version: String,
    pub installed: bool,
}

pub struct Launcher {
    applications: BTreeMap<String, Application>,
    recent_apps: Vec<String>,
}

impl Launcher {
    pub fn new() -> Self {
        Self {
            applications: BTreeMap::new(),
            recent_apps: Vec::new(),
        }
    }

    pub fn register_app(&mut self, app: Application) {
        self.applications
            .insert(app.package_name.clone(), app);
    }

    pub fn uninstall_app(&mut self, package: &str) {
        self.applications.remove(package);
    }

    pub fn launch_app(&mut self, package: &str) -> Result<(), &'static str> {
        let app = self
            .applications
            .get(package)
            .ok_or("application not found")?;

        if !app.installed {
            return Err("application not installed");
        }

        println!("Launching {}", app.name);

        self.recent_apps.push(package.to_string());

        if self.recent_apps.len() > 25 {
            self.recent_apps.remove(0);
        }

        Ok(())
    }

    pub fn search(&self, query: &str) -> Vec<&Application> {
        let query = query.to_lowercase();

        self.applications
            .values()
            .filter(|app| {
                app.name.to_lowercase().contains(&query)
                    || app.package_name.to_lowercase().contains(&query)
            })
            .collect()
    }

    pub fn list_installed(&self) -> Vec<&Application> {
        self.applications.values().collect()
    }

    pub fn recent_apps(&self) -> &[String] {
        &self.recent_apps
    }
}

impl Default for Launcher {
    fn default() -> Self {
        Self::new()
    }
}
