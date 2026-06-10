#[derive(Clone)]
pub enum UpdateChannel {
    Stable,
    Beta,
    Developer,
}

#[derive(Clone)]
pub struct SystemUpdate {
    pub version: String,
    pub mandatory: bool,
    pub size_mb: u64,
}

pub struct UpdateManager {
    channel: UpdateChannel,
    pending_update: Option<SystemUpdate>,
    auto_install: bool,
}

impl UpdateManager {
    pub fn new() -> Self {
        Self {
            channel: UpdateChannel::Stable,
            pending_update: None,
            auto_install: false,
        }
    }

    pub fn set_channel(&mut self, channel: UpdateChannel) {
        self.channel = channel;
    }

    pub fn queue_update(&mut self, update: SystemUpdate) {
        self.pending_update = Some(update);
    }

    pub fn install_update(&mut self) -> bool {
        if let Some(update) = &self.pending_update {
            println!("Installing update: {}", update.version);
            self.pending_update = None;
            return true;
        }
        false
    }

    pub fn has_update(&self) -> bool {
        self.pending_update.is_some()
    }
}
