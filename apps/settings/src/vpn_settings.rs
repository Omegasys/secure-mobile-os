#[derive(Clone)]
pub enum VpnProtocol {
    WireGuard,
    OpenVpn,
    IpSec,
    Custom,
}

pub struct VpnSettings {
    pub enabled: bool,

    pub protocol: VpnProtocol,

    pub always_on: bool,

    pub kill_switch_enabled: bool,

    pub split_tunneling_enabled: bool,

    pub server_address: String,
}

impl VpnSettings {
    pub fn new() -> Self {
        Self {
            enabled: false,

            protocol: VpnProtocol::WireGuard,

            always_on: false,

            kill_switch_enabled: true,

            split_tunneling_enabled: false,

            server_address:
                "vpn.example.local".to_string(),
        }
    }

    pub fn toggle_vpn(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn toggle_always_on(&mut self) {
        self.always_on = !self.always_on;
    }

    pub fn toggle_kill_switch(
        &mut self,
    ) {
        self.kill_switch_enabled =
            !self.kill_switch_enabled;
    }

    pub fn toggle_split_tunnel(
        &mut self,
    ) {
        self.split_tunneling_enabled =
            !self.split_tunneling_enabled;
    }

    pub fn set_protocol(
        &mut self,
        protocol: VpnProtocol,
    ) {
        self.protocol = protocol;
    }

    pub fn set_server(
        &mut self,
        server: String,
    ) {
        self.server_address = server;
    }

    pub fn print_status(&self) {
        println!("--- VPN Settings ---");

        println!("Enabled: {}", self.enabled);
        println!("Always On: {}", self.always_on);

        println!(
            "Kill Switch: {}",
            self.kill_switch_enabled
        );

        println!(
            "Split Tunnel: {}",
            self.split_tunneling_enabled
        );

        println!(
            "Server: {}",
            self.server_address
        );

        match self.protocol {
            VpnProtocol::WireGuard =>
                println!("Protocol: WireGuard"),

            VpnProtocol::OpenVpn =>
                println!("Protocol: OpenVPN"),

            VpnProtocol::IpSec =>
                println!("Protocol: IPsec"),

            VpnProtocol::Custom =>
                println!("Protocol: Custom"),
        }
    }
}

impl Default for VpnSettings {
    fn default() -> Self {
        Self::new()
    }
}
