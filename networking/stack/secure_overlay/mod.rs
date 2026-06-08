pub mod routing;
pub mod encryption;
pub mod key_exchange;
pub mod relay_nodes;

pub const OVERLAY_VERSION: u16 = 1;

pub struct OverlayPacket {
    pub version: u16,
    pub session_id: u64,
    pub payload_size: usize,
}

impl OverlayPacket {
    pub fn new(
        session_id: u64,
        payload_size: usize,
    ) -> Self {
        Self {
            version: OVERLAY_VERSION,
            session_id,
            payload_size,
        }
    }
}
