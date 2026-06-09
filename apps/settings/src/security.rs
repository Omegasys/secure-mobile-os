#[derive(Clone)]
pub struct SecuritySettings {
    pub firewall_enabled: bool,
    pub secure_boot_enabled: bool,
    pub encrypted_storage_enabled: bool,

    pub vpn_required: bool,

    pub intrusion_detection_enabled: bool,

    pub audit_logging_enabled: bool,
}

impl SecuritySettings {
    pub fn new() -> Self {
        Self {
            firewall_enabled: true,
            secure_boot_enabled: true,
            encrypted_storage_enabled: true,

            vpn_required: false,

            intrusion_detection_enabled: true,

            audit_logging_enabled: true,
        }
    }

    pub fn toggle_firewall(&mut self) {
        self.firewall_enabled =
            !self.firewall_enabled;
    }

    pub fn toggle_secure_boot(&mut self) {
        self.secure_boot_enabled =
            !self.secure_boot_enabled;
    }

    pub fn toggle_storage_encryption(&mut self) {
        self.encrypted_storage_enabled =
            !self.encrypted_storage_enabled;
    }

    pub fn toggle_vpn_requirement(&mut self) {
        self.vpn_required =
            !self.vpn_required;
    }

    pub fn toggle_intrusion_detection(&mut self) {
        self.intrusion_detection_enabled =
            !self.intrusion_detection_enabled;
    }

    pub fn toggle_audit_logging(&mut self) {
        self.audit_logging_enabled =
            !self.audit_logging_enabled;
    }

    pub fn print_status(&self) {
        println!("--- Security Settings ---");
        println!("Firewall: {}", self.firewall_enabled);
        println!("Secure Boot: {}", self.secure_boot_enabled);
        println!(
            "Encrypted Storage: {}",
            self.encrypted_storage_enabled
        );
        println!("VPN Required: {}", self.vpn_required);
        println!(
            "Intrusion Detection: {}",
            self.intrusion_detection_enabled
        );
        println!(
            "Audit Logging: {}",
            self.audit_logging_enabled
        );
    }
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self::new()
    }
}
