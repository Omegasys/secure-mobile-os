#[derive(Clone)]
pub struct VpnProfile {
    pub name: String,
    pub endpoint: String,
}

pub struct VpnDashboard {
    enabled: bool,
    active_profile: Option<VpnProfile>,
}

impl VpnDashboard {
    pub fn new() -> Self {
        Self {
            enabled: false,
            active_profile: None,
        }
    }

    pub fn connect(
        &mut self,
        profile: VpnProfile,
    ) {
        self.active_profile = Some(profile);
        self.enabled = true;
    }

    pub fn disconnect(&mut self) {
        self.active_profile = None;
        self.enabled = false;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn active_profile(
        &self,
    ) -> Option<&VpnProfile> {
        self.active_profile.as_ref()
    }
}

impl Default for VpnDashboard {
    fn default() -> Self {
        Self::new()
    }
}
