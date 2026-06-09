#[derive(Clone)]
pub enum UpdateChannel {
    Stable,
    Beta,
    Developer,
}

pub struct UpdateSettings {
    pub automatic_updates: bool,
    pub rollback_enabled: bool,

    pub update_channel: UpdateChannel,

    pub current_version: String,
    pub available_version: Option<String>,
}

impl UpdateSettings {
    pub fn new() -> Self {
        Self {
            automatic_updates: true,
            rollback_enabled: true,

            update_channel: UpdateChannel::Stable,

            current_version: "1.0.0".to_string(),
            available_version: None,
        }
    }

    pub fn set_channel(
        &mut self,
        channel: UpdateChannel,
    ) {
        self.update_channel = channel;
    }

    pub fn toggle_auto_updates(&mut self) {
        self.automatic_updates =
            !self.automatic_updates;
    }

    pub fn toggle_rollback(&mut self) {
        self.rollback_enabled =
            !self.rollback_enabled;
    }

    pub fn check_for_updates(&mut self) {
        println!("Checking for updates...");

        self.available_version =
            Some("1.1.0".to_string());
    }

    pub fn install_update(&mut self) {
        if let Some(version) =
            self.available_version.take()
        {
            println!("Installing {}", version);
            self.current_version = version;
        }
    }

    pub fn print_status(&self) {
        println!("--- Update Settings ---");
        println!("Current Version: {}", self.current_version);
        println!("Automatic Updates: {}", self.automatic_updates);
        println!("Rollback Enabled: {}", self.rollback_enabled);

        if let Some(version) =
            &self.available_version
        {
            println!("Available Version: {}", version);
        }
    }
}

impl Default for UpdateSettings {
    fn default() -> Self {
        Self::new()
    }
}
