mod firewall;
mod vpn;
mod overlay_network;
mod permissions;

use firewall::FirewallDashboard;
use vpn::VpnDashboard;
use overlay_network::OverlayDashboard;
use permissions::PermissionDashboard;

pub struct SecurityDashboard {
    pub firewall: FirewallDashboard,
    pub vpn: VpnDashboard,
    pub overlay: OverlayDashboard,
    pub permissions: PermissionDashboard,
}

impl SecurityDashboard {
    pub fn new() -> Self {
        Self {
            firewall: FirewallDashboard::new(),
            vpn: VpnDashboard::new(),
            overlay: OverlayDashboard::new(),
            permissions: PermissionDashboard::new(),
        }
    }

    pub fn show_overview(&self) {
        println!("=== Security Dashboard ===");

        println!(
            "Firewall: {}",
            if self.firewall.enabled() {
                "Enabled"
            } else {
                "Disabled"
            }
        );

        println!(
            "VPN: {}",
            if self.vpn.enabled() {
                "Connected"
            } else {
                "Disconnected"
            }
        );

        println!(
            "Overlay Network: {}",
            if self.overlay.enabled() {
                "Enabled"
            } else {
                "Disabled"
            }
        );

        println!(
            "Applications Tracked: {}",
            self.permissions.app_count()
        );
    }
}

fn main() {
    let dashboard = SecurityDashboard::new();
    dashboard.show_overview();
}
