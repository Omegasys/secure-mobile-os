use std::collections::VecDeque;

#[derive(Clone)]
pub struct RelayNode {
    pub id: String,
    pub address: String,
}

#[derive(Clone)]
pub struct TransportPacket {
    pub packet_id: u64,
    pub destination: String,
    pub payload_size: usize,
}

pub struct OverlayTransport {
    relays: Vec<RelayNode>,
    outbound_queue: VecDeque<TransportPacket>,
}

impl OverlayTransport {
    pub fn new() -> Self {
        Self {
            relays: Vec::new(),
            outbound_queue: VecDeque::new(),
        }
    }

    pub fn add_relay(
        &mut self,
        relay: RelayNode,
    ) {
        self.relays.push(relay);
    }

    pub fn remove_relay(
        &mut self,
        relay_id: &str,
    ) {
        self.relays
            .retain(|r| r.id != relay_id);
    }

    pub fn queue_packet(
        &mut self,
        packet: TransportPacket,
    ) {
        self.outbound_queue.push_back(packet);
    }

    pub fn next_packet(
        &mut self,
    ) -> Option<TransportPacket> {
        self.outbound_queue.pop_front()
    }

    pub fn relay_count(&self) -> usize {
        self.relays.len()
    }

    pub fn queued_packets(
        &self,
    ) -> usize {
        self.outbound_queue.len()
    }

    pub fn relays(&self) -> &[RelayNode] {
        &self.relays
    }
}

impl Default for OverlayTransport {
    fn default() -> Self {
        Self::new()
    }
}
