use super::packet_filter::Packet;

pub struct IntrusionEvent {
    pub severity: u8,
    pub description: &'static str,
}

pub struct IntrusionDetectionSystem {
    packet_count: usize,
}

impl IntrusionDetectionSystem {
    pub const fn new() -> Self {
        Self { packet_count: 0 }
    }

    pub fn analyze(&mut self, packet: &Packet) -> Option<IntrusionEvent> {
        self.packet_count += 1;

        // Simple heuristic-based detection (placeholder)
        if packet.size > 8000 {
            return Some(IntrusionEvent {
                severity: 90,
                description: "Oversized packet anomaly detected",
            });
        }

        if self.packet_count % 10000 == 0 {
            return Some(IntrusionEvent {
                severity: 10,
                description: "High traffic volume spike",
            });
        }

        None
    }
}
