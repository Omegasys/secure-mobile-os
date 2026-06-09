pub struct TelemetrySettings {
    pub telemetry_enabled: bool,

    pub crash_reports_enabled: bool,

    pub anonymous_usage_enabled: bool,

    pub security_audit_uploads: bool,

    pub local_logging_enabled: bool,

    pub data_retention_days: u32,
}

impl TelemetrySettings {
    pub fn new() -> Self {
        Self {
            telemetry_enabled: false,

            crash_reports_enabled: true,

            anonymous_usage_enabled: false,

            security_audit_uploads: false,

            local_logging_enabled: true,

            data_retention_days: 30,
        }
    }

    pub fn toggle_telemetry(
        &mut self,
    ) {
        self.telemetry_enabled =
            !self.telemetry_enabled;
    }

    pub fn toggle_crash_reports(
        &mut self,
    ) {
        self.crash_reports_enabled =
            !self.crash_reports_enabled;
    }

    pub fn toggle_usage_collection(
        &mut self,
    ) {
        self.anonymous_usage_enabled =
            !self.anonymous_usage_enabled;
    }

    pub fn toggle_audit_uploads(
        &mut self,
    ) {
        self.security_audit_uploads =
            !self.security_audit_uploads;
    }

    pub fn toggle_local_logging(
        &mut self,
    ) {
        self.local_logging_enabled =
            !self.local_logging_enabled;
    }

    pub fn set_retention_days(
        &mut self,
        days: u32,
    ) {
        self.data_retention_days = days;
    }

    pub fn print_status(&self) {
        println!("--- Telemetry Settings ---");

        println!(
            "Telemetry Enabled: {}",
            self.telemetry_enabled
        );

        println!(
            "Crash Reports: {}",
            self.crash_reports_enabled
        );

        println!(
            "Anonymous Usage: {}",
            self.anonymous_usage_enabled
        );

        println!(
            "Audit Uploads: {}",
            self.security_audit_uploads
        );

        println!(
            "Local Logging: {}",
            self.local_logging_enabled
        );

        println!(
            "Retention Days: {}",
            self.data_retention_days
        );
    }
}

impl Default for TelemetrySettings {
    fn default() -> Self {
        Self::new()
    }
}
