#[derive(Clone)]
pub struct PrivacySettings {
    pub telemetry_enabled: bool,
    pub crash_reports_enabled: bool,

    pub camera_access_enabled: bool,
    pub microphone_access_enabled: bool,
    pub location_access_enabled: bool,

    pub advertising_id_enabled: bool,
    pub analytics_enabled: bool,
}

impl PrivacySettings {
    pub fn new() -> Self {
        Self {
            telemetry_enabled: false,
            crash_reports_enabled: true,

            camera_access_enabled: true,
            microphone_access_enabled: true,
            location_access_enabled: true,

            advertising_id_enabled: false,
            analytics_enabled: false,
        }
    }

    pub fn toggle_telemetry(&mut self) {
        self.telemetry_enabled = !self.telemetry_enabled;
    }

    pub fn toggle_crash_reports(&mut self) {
        self.crash_reports_enabled =
            !self.crash_reports_enabled;
    }

    pub fn toggle_camera_access(&mut self) {
        self.camera_access_enabled =
            !self.camera_access_enabled;
    }

    pub fn toggle_microphone_access(&mut self) {
        self.microphone_access_enabled =
            !self.microphone_access_enabled;
    }

    pub fn toggle_location_access(&mut self) {
        self.location_access_enabled =
            !self.location_access_enabled;
    }

    pub fn toggle_advertising_id(&mut self) {
        self.advertising_id_enabled =
            !self.advertising_id_enabled;
    }

    pub fn toggle_analytics(&mut self) {
        self.analytics_enabled =
            !self.analytics_enabled;
    }

    pub fn print_status(&self) {
        println!("--- Privacy Settings ---");
        println!("Telemetry: {}", self.telemetry_enabled);
        println!("Crash Reports: {}", self.crash_reports_enabled);
        println!("Camera: {}", self.camera_access_enabled);
        println!("Microphone: {}", self.microphone_access_enabled);
        println!("Location: {}", self.location_access_enabled);
        println!("Advertising ID: {}", self.advertising_id_enabled);
        println!("Analytics: {}", self.analytics_enabled);
    }
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self::new()
    }
}
