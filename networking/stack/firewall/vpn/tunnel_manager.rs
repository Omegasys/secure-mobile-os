use crate::networking::stack::secure_overlay::encryption::SessionKey;

pub struct VpnTunnel {
    pub active: bool,
    pub latency_ms: u32,
}

pub struct TunnelManager {
    tunnels: [Option<VpnTunnel>; 16],
    key: Option<SessionKey>,
}

impl TunnelManager {
    pub const fn new() -> Self {
        Self {
            tunnels: [None; 16],
            key: None,
        }
    }

    pub fn establish_tunnel(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= 16 {
            return Err("invalid tunnel slot");
        }

        self.tunnels[index] = Some(VpnTunnel {
            active: true,
            latency_ms: 0,
        });

        Ok(())
    }

    pub fn close_tunnel(&mut self, index: usize) {
        if let Some(t) = &mut self.tunnels[index] {
            t.active = false;
        }
    }

    pub fn set_key(&mut self, key: SessionKey) {
        self.key = Some(key);
    }
}
