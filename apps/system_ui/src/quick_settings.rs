#[derive(Clone, Copy)]
pub struct QuickToggle {
    pub enabled: bool,
}

pub struct QuickSettings {
    pub wifi: QuickToggle,
    pub bluetooth: QuickToggle,
    pub mobile_data: QuickToggle,
    pub vpn: QuickToggle,
    pub airplane_mode: QuickToggle,
    pub flashlight: QuickToggle,
    pub microphone_access: QuickToggle,
    pub camera_access: QuickToggle,
    pub location_access: QuickToggle,
}

impl QuickSettings {
    pub fn new() -> Self {
        Self {
            wifi: QuickToggle { enabled: true },
            bluetooth: QuickToggle { enabled: false },
            mobile_data: QuickToggle { enabled: true },
            vpn: QuickToggle { enabled: false },
            airplane_mode: QuickToggle { enabled: false },
            flashlight: QuickToggle { enabled: false },
            microphone_access: QuickToggle { enabled: true },
            camera_access: QuickToggle { enabled: true },
            location_access: QuickToggle { enabled: true },
        }
    }

    pub fn toggle_wifi(&mut self) {
        self.wifi.enabled = !self.wifi.enabled;
    }

    pub fn toggle_bluetooth(&mut self) {
        self.bluetooth.enabled = !self.bluetooth.enabled;
    }

    pub fn toggle_mobile_data(&mut self) {
        self.mobile_data.enabled = !self.mobile_data.enabled;
    }

    pub fn toggle_vpn(&mut self) {
        self.vpn.enabled = !self.vpn.enabled;
    }

    pub fn toggle_airplane_mode(&mut self) {
        self.airplane_mode.enabled = !self.airplane_mode.enabled;

        if self.airplane_mode.enabled {
            self.wifi.enabled = false;
            self.bluetooth.enabled = false;
            self.mobile_data.enabled = false;
        }
    }

    pub fn toggle_flashlight(&mut self) {
        self.flashlight.enabled = !self.flashlight.enabled;
    }

    pub fn toggle_microphone(&mut self) {
        self.microphone_access.enabled =
            !self.microphone_access.enabled;
    }

    pub fn toggle_camera(&mut self) {
        self.camera_access.enabled =
            !self.camera_access.enabled;
    }

    pub fn toggle_location(&mut self) {
        self.location_access.enabled =
            !self.location_access.enabled;
    }

    pub fn reset_defaults(&mut self) {
        *self = Self::new();
    }
}

impl Default for QuickSettings {
    fn default() -> Self {
        Self::new()
    }
}
