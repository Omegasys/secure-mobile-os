mod network;
mod display;

use network::NetworkSettings;
use display::DisplaySettings;

pub struct SettingsApp {
    pub network: NetworkSettings,
    pub display: DisplaySettings,
}

impl SettingsApp {
    pub fn new() -> Self {
        Self {
            network: NetworkSettings::new(),
            display: DisplaySettings::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Settings app started");

        self.network.print_status();
        self.display.print_status();
    }
}

fn main() {
    let mut app = SettingsApp::new();
    app.run();
}
