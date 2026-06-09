#[derive(Clone)]
pub struct Ipv6Settings {
    pub enabled: bool,

    pub stateless_autoconfig: bool,

    pub address: String,
    pub gateway: String,

    pub dns_primary: String,
    pub dns_secondary: String,

    pub privacy_extensions: bool,
}

impl Ipv6Settings {
    pub fn new() -> Self {
        Self {
            enabled: true,

            stateless_autoconfig: true,

            address: "::".to_string(),
            gateway: "::".to_string(),

            dns_primary:
                "2606:4700:4700::1111".to_string(),

            dns_secondary:
                "2606:4700:4700::1001".to_string(),

            privacy_extensions: true,
        }
    }

    pub fn toggle_ipv6(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn toggle_autoconfig(&mut self) {
        self.stateless_autoconfig =
            !self.stateless_autoconfig;
    }

    pub fn toggle_privacy_extensions(
        &mut self,
    ) {
        self.privacy_extensions =
            !self.privacy_extensions;
    }

    pub fn set_static_configuration(
        &mut self,
        address: String,
        gateway: String,
    ) {
        self.address = address;
        self.gateway = gateway;
    }

    pub fn print_status(&self) {
        println!("--- IPv6 Settings ---");
        println!("Enabled: {}", self.enabled);

        println!(
            "Auto Configure: {}",
            self.stateless_autoconfig
        );

        println!(
            "Privacy Extensions: {}",
            self.privacy_extensions
        );

        println!("Address: {}", self.address);
        println!("Gateway: {}", self.gateway);
        println!("DNS1: {}", self.dns_primary);
        println!("DNS2: {}", self.dns_secondary);
    }
}

impl Default for Ipv6Settings {
    fn default() -> Self {
        Self::new()
    }
}
