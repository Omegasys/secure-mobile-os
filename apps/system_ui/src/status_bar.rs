use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy)]
pub enum NetworkType {
    None,
    Wifi,
    Mobile4G,
    Mobile5G,
    Ethernet,
    SecureOverlay,
}

pub struct StatusBar {
    battery_percentage: u8,
    charging: bool,

    signal_strength: u8,
    network_type: NetworkType,

    vpn_active: bool,
    bluetooth_enabled: bool,

    notification_count: usize,

    clock_timestamp: u64,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            battery_percentage: 100,
            charging: false,

            signal_strength: 5,
            network_type: NetworkType::Wifi,

            vpn_active: false,
            bluetooth_enabled: false,

            notification_count: 0,

            clock_timestamp: current_time(),
        }
    }

    pub fn set_battery(
        &mut self,
        percentage: u8,
        charging: bool,
    ) {
        self.battery_percentage = percentage.min(100);
        self.charging = charging;
    }

    pub fn set_network(
        &mut self,
        network: NetworkType,
        strength: u8,
    ) {
        self.network_type = network;
        self.signal_strength = strength.min(5);
    }

    pub fn set_vpn_status(&mut self, active: bool) {
        self.vpn_active = active;
    }

    pub fn set_bluetooth_status(&mut self, enabled: bool) {
        self.bluetooth_enabled = enabled;
    }

    pub fn set_notification_count(&mut self, count: usize) {
        self.notification_count = count;
    }

    pub fn refresh_clock(&mut self) {
        self.clock_timestamp = current_time();
    }

    pub fn battery_percentage(&self) -> u8 {
        self.battery_percentage
    }

    pub fn charging(&self) -> bool {
        self.charging
    }

    pub fn signal_strength(&self) -> u8 {
        self.signal_strength
    }

    pub fn vpn_active(&self) -> bool {
        self.vpn_active
    }

    pub fn bluetooth_enabled(&self) -> bool {
        self.bluetooth_enabled
    }

    pub fn notification_count(&self) -> usize {
        self.notification_count
    }

    pub fn timestamp(&self) -> u64 {
        self.clock_timestamp
    }
}

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}
