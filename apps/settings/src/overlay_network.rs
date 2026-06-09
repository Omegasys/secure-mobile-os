#[derive(Clone)]
pub enum OverlayMode {
    Disabled,
    Direct,
    SingleRelay,
    MultiHop,
}

pub struct OverlayNetworkSettings {
    pub enabled: bool,

    pub mode: OverlayMode,

    pub relay_count: u8,

    pub automatic_relay_selection: bool,

    pub encryption_enabled: bool,

    pub socks5_bridge_enabled: bool,
}

impl OverlayNetworkSettings {
    pub fn new() -> Self {
        Self {
            enabled: true,

            mode: OverlayMode::MultiHop,

            relay_count: 2,

            automatic_relay_selection: true,

            encryption_enabled: true,

            socks5_bridge_enabled: false,
        }
    }

    pub fn toggle_overlay(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn set_mode(
        &mut self,
        mode: OverlayMode,
    ) {
        self.mode = mode;
    }

    pub fn set_relay_count(
        &mut self,
        count: u8,
    ) {
        self.relay_count = count.max(1);
    }

    pub fn toggle_auto_relay_selection(
        &mut self,
    ) {
        self.automatic_relay_selection =
            !self.automatic_relay_selection;
    }

    pub fn toggle_encryption(&mut self) {
        self.encryption_enabled =
            !self.encryption_enabled;
    }

    pub fn toggle_socks5_bridge(
        &mut self,
    ) {
        self.socks5_bridge_enabled =
            !self.socks5_bridge_enabled;
    }

    pub fn print_status(&self) {
        println!("--- Overlay Network ---");

        println!("Enabled: {}", self.enabled);
        println!("Relay Count: {}", self.relay_count);

        println!(
            "Automatic Relay Selection: {}",
            self.automatic_relay_selection
        );

        println!(
            "Encryption Enabled: {}",
            self.encryption_enabled
        );

        println!(
            "SOCKS5 Bridge: {}",
            self.socks5_bridge_enabled
        );

        match self.mode {
            OverlayMode::Disabled =>
                println!("Mode: Disabled"),

            OverlayMode::Direct =>
                println!("Mode: Direct"),

            OverlayMode::SingleRelay =>
                println!("Mode: Single Relay"),

            OverlayMode::MultiHop =>
                println!("Mode: Multi-Hop"),
        }
    }
}

impl Default for OverlayNetworkSettings {
    fn default() -> Self {
        Self::new()
    }
}
