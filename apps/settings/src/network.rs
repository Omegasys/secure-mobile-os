#[derive(Clone)]
pub enum IpMode {
    Ipv4Only,
    Ipv6Only,
    DualStack,
    Disabled,
}

#[derive(Clone)]
pub struct NetworkSettings {
    pub wifi_enabled: bool,
    pub mobile_data_enabled: bool,
    pub vpn_enabled: bool,

    pub ip_mode: IpMode,
    pub secure_overlay_enabled: bool,
    pub secure_dns_enabled: bool,
}

impl NetworkSettings {
    pub fn new() -> Self {
        Self {
            wifi_enabled: true,
            mobile_data_enabled: true,
            vpn_enabled: false,

            ip_mode: IpMode::DualStack,
            secure_overlay_enabled: true,
            secure_dns_enabled: true,
        }
    }

    pub fn set_ip_mode(&mut self, mode: IpMode) {
        self.ip_mode = mode;
    }

    pub fn toggle_wifi(&mut self) {
        self.wifi_enabled = !self.wifi_enabled;
    }

    pub fn toggle_mobile_data(&mut self) {
        self.mobile_data_enabled = !self.mobile_data_enabled;
    }

    pub fn toggle_vpn(&mut self) {
        self.vpn_enabled = !self.vpn_enabled;
    }

    pub fn toggle_secure_overlay(&mut self) {
        self.secure_overlay_enabled = !self.secure_overlay_enabled;
    }

    pub fn toggle_secure_dns(&mut self) {
        self.secure_dns_enabled = !self.secure_dns_enabled;
    }

    pub fn print_status(&self) {
        println!("--- Network Settings ---");
        println!("WiFi: {}", self.wifi_enabled);
        println!("Mobile Data: {}", self.mobile_data_enabled);
        println!("VPN: {}", self.vpn_enabled);
        println!("Secure Overlay: {}", self.secure_overlay_enabled);
        println!("Secure DNS: {}", self.secure_dns_enabled);

        match self.ip_mode {
            IpMode::Ipv4Only => println!("IP Mode: IPv4 Only"),
            IpMode::Ipv6Only => println!("IP Mode: IPv6 Only"),
            IpMode::DualStack => println!("IP Mode: Dual Stack"),
            IpMode::Disabled => println!("IP Mode: Disabled"),
        }
    }
}
