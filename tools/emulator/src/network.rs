```rust
#[derive(Debug, Clone)]
pub enum ConnectionType {
    Disconnected,
    Ethernet,
    Wifi,
    Cellular,
}

pub struct VirtualNetwork {
    pub connection_type: ConnectionType,
    pub ip_address: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

impl VirtualNetwork {
    pub fn new() -> Self {
        Self {
            connection_type: ConnectionType::Disconnected,
            ip_address: "0.0.0.0".to_string(),
            bytes_sent: 0,
            bytes_received: 0,
        }
    }

    pub fn connect(
        &mut self,
        connection_type: ConnectionType,
        ip_address: String,
    ) {
        self.connection_type = connection_type;
        self.ip_address = ip_address;

        println!(
            "Network connected: {}",
            self.ip_address
        );
    }

    pub fn disconnect(&mut self) {
        self.connection_type = ConnectionType::Disconnected;
        self.ip_address = "0.0.0.0".to_string();

        println!("Network disconnected");
    }

    pub fn send_data(&mut self, bytes: u64) {
        self.bytes_sent += bytes;
    }

    pub fn receive_data(&mut self, bytes: u64) {
        self.bytes_received += bytes;
    }

    pub fn print_info(&self) {
        println!("Network Information");
        println!("Type: {:?}", self.connection_type);
        println!("IP Address: {}", self.ip_address);
        println!("Sent: {} bytes", self.bytes_sent);
        println!("Received: {} bytes", self.bytes_received);
    }
}
```
