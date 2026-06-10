#[derive(Clone)]
pub enum OverlayMode {
    Direct,
    SingleRelay,
    MultiHop,
}

pub struct OverlayDashboard {
    enabled: bool,
    mode: OverlayMode,
    relay_count: u8,
}

impl OverlayDashboard {
    pub fn new() -> Self {
        Self {
            enabled: true,
            mode: OverlayMode::MultiHop,
            relay_count: 2,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_mode(
        &mut self,
        mode: OverlayMode,
    ) {
        self.mode = mode;
    }

    pub fn relay_count(&self) -> u8 {
        self.relay_count
    }

    pub fn set_relay_count(
        &mut self,
        count: u8,
    ) {
        self.relay_count = count.max(1);
    }
}

impl Default for OverlayDashboard {
    fn default() -> Self {
        Self::new()
    }
}
