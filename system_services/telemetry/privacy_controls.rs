#[derive(Clone, Copy)]
pub enum TelemetryLevel {
    Off,
    Minimal,
    Standard,
    Diagnostic,
}

pub struct PrivacyControls {
    level: TelemetryLevel,
    allow_crash_reports: bool,
    allow_usage_stats: bool,
}

impl PrivacyControls {
    pub const fn new() -> Self {
        Self {
            level: TelemetryLevel::Off,
            allow_crash_reports: false,
            allow_usage_stats: false,
        }
    }

    pub fn set_level(&mut self, level: TelemetryLevel) {
        self.level = level;

        match level {
            TelemetryLevel::Off => {
                self.allow_crash_reports = false;
                self.allow_usage_stats = false;
            }
            TelemetryLevel::Minimal => {
                self.allow_crash_reports = true;
                self.allow_usage_stats = false;
            }
            TelemetryLevel::Standard => {
                self.allow_crash_reports = true;
                self.allow_usage_stats = true;
            }
            TelemetryLevel::Diagnostic => {
                self.allow_crash_reports = true;
                self.allow_usage_stats = true;
            }
        }
    }

    pub fn telemetry_enabled(&self) -> bool {
        self.allow_crash_reports || self.allow_usage_stats
    }

    pub fn level(&self) -> TelemetryLevel {
        self.level
    }
}
