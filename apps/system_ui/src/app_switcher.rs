#[derive(Clone)]
pub struct RunningApp {
    pub pid: u64,
    pub app_name: String,
    pub package_name: String,
    pub memory_usage_mb: usize,
    pub suspended: bool,
}

pub struct AppSwitcher {
    running_apps: Vec<RunningApp>,
    focused_app: Option<u64>,
}

impl AppSwitcher {
    pub fn new() -> Self {
        Self {
            running_apps: Vec::new(),
            focused_app: None,
        }
    }

    pub fn register_app(&mut self, app: RunningApp) {
        self.focused_app = Some(app.pid);
        self.running_apps.push(app);
    }

    pub fn switch_to(&mut self, pid: u64) -> Result<(), &'static str> {
        let exists = self
            .running_apps
            .iter()
            .any(|app| app.pid == pid);

        if !exists {
            return Err("application not found");
        }

        self.focused_app = Some(pid);

        for app in &mut self.running_apps {
            app.suspended = app.pid != pid;
        }

        Ok(())
    }

    pub fn close_app(&mut self, pid: u64) {
        self.running_apps.retain(|app| app.pid != pid);

        if self.focused_app == Some(pid) {
            self.focused_app = self
                .running_apps
                .last()
                .map(|app| app.pid);
        }
    }

    pub fn suspend_background_apps(&mut self) {
        for app in &mut self.running_apps {
            if Some(app.pid) != self.focused_app {
                app.suspended = true;
            }
        }
    }

    pub fn resume_app(&mut self, pid: u64) {
        for app in &mut self.running_apps {
            if app.pid == pid {
                app.suspended = false;
            }
        }
    }

    pub fn running_apps(&self) -> &[RunningApp] {
        &self.running_apps
    }

    pub fn focused_app(&self) -> Option<u64> {
        self.focused_app
    }

    pub fn total_memory_usage(&self) -> usize {
        self.running_apps
            .iter()
            .map(|app| app.memory_usage_mb)
            .sum()
    }

    pub fn app_count(&self) -> usize {
        self.running_apps.len()
    }
}

impl Default for AppSwitcher {
    fn default() -> Self {
        Self::new()
    }
}
