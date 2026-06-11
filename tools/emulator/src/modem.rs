```rust
#[derive(Debug)]
pub enum NetworkType {
    None,
    LTE,
    FiveG,
    WifiCalling,
}

pub struct VirtualModem {
    pub carrier: String,
    pub signal_strength: u8,
    pub network_type: NetworkType,
}

impl VirtualModem {
    pub fn new() -> Self {
        Self {
            carrier: "Secure Mobile".to_string(),
            signal_strength: 100,
            network_type: NetworkType::FiveG,
        }
    }

    pub fn connect(&self) {
        println!(
            "Connected to {} via {:?}",
            self.carrier,
            self.network_type
        );
    }

    pub fn disconnect(&self) {
        println!("Modem disconnected");
    }

    pub fn print_info(&self) {
        println!("Modem");
        println!("Carrier: {}", self.carrier);
        println!(
            "Signal: {}%",
            self.signal_strength
        );
        println!(
            "Network: {:?}",
            self.network_type
        );
    }
}
```
