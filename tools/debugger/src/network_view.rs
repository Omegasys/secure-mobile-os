```rust
#[derive(Debug)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_address: String,
    pub mtu: u32,
    pub up: bool,
}

#[derive(Debug)]
pub struct SocketInfo {
    pub protocol: String,
    pub local_address: String,
    pub remote_address: String,
    pub state: String,
}

pub fn get_interfaces() -> Vec<NetworkInterface> {
    vec![
        NetworkInterface {
            name: "lo".to_string(),
            ip_address: "127.0.0.1".to_string(),
            mtu: 65536,
            up: true,
        },
        NetworkInterface {
            name: "wlan0".to_string(),
            ip_address: "192.168.1.100".to_string(),
            mtu: 1500,
            up: true,
        },
    ]
}

pub fn get_sockets() -> Vec<SocketInfo> {
    vec![
        SocketInfo {
            protocol: "TCP".to_string(),
            local_address: "192.168.1.100:443".to_string(),
            remote_address: "203.0.113.1:50123".to_string(),
            state: "ESTABLISHED".to_string(),
        }
    ]
}

pub fn show_network() {
    println!("Network Interfaces");
    println!("------------------");

    for iface in get_interfaces() {
        println!(
            "{} {} mtu={} up={}",
            iface.name,
            iface.ip_address,
            iface.mtu,
            iface.up
        );
    }

    println!();
    println!("Sockets");
    println!("-------");

    for socket in get_sockets() {
        println!(
            "{} {} -> {} ({})",
            socket.protocol,
            socket.local_address,
            socket.remote_address,
            socket.state
        );
    }
}
```
