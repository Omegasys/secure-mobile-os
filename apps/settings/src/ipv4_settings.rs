#[derive(Clone)]
pub struct Ipv4Settings {
    pub enabled: bool,

    pub dhcp_enabled: bool,

    pub address: String,
    pub subnet_mask: String,
    pub gateway: String,

    pub dns_primary: String,
    pub dns_secondary: String,
}

impl Ipv4Settings {
    pub fn new() -> Self {
        Self {
            enabled: true,

            dhcp_enabled: true,

            address: "0.0.0.0".to_string(),
            subnet_mask: "255.255.255.0".to_string(),
            gateway: "0.0.0.0".to_string(),

            dns_primary: "1.1.1.1".to_string(),
            dns_secondary: "8.8.8.8".to_string(),
        }
    }

    pub fn toggle_ipv4(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn toggle_dhcp(&mut self) {
        self.dhcp_enabled = !self.dhcp_enabled;
    }

    pub fn set_static_configuration(
        &mut self,
        address: String,
        subnet: String,
        gateway: String,
    ) {
        self.address = address;
        self.subnet_mask = subnet;
        self.gateway = gateway;
    }

    pub fn print_status(&self) {
        println!("--- IPv4 Settings ---");
        println!("Enabled: {}", self.enabled);
        println!("DHCP: {}", self.dhcp_enabled);
        println!("Address: {}", self.address);
        println!("Subnet: {}", self.subnet_mask);
        println!("Gateway: {}", self.gateway);
        println!("DNS1: {}", self.dns_primary);
        println!("DNS2: {}", self.dns_secondary);
    }
}

impl Default for Ipv4Settings {
    fn default() -> Self {
        Self::new()
    }
}
