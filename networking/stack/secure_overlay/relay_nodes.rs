pub const MAX_RELAYS: usize = 1024;

#[derive(Clone, Copy)]
pub struct RelayNode {
    pub id: u64,
    pub latency_ms: u32,
    pub trust_score: u8,
}

pub struct RelayDirectory {
    pub count: usize,
    pub nodes: [Option<RelayNode>; MAX_RELAYS],
}

impl RelayDirectory {
    pub const fn new() -> Self {
        Self {
            count: 0,
            nodes: [None; MAX_RELAYS],
        }
    }

    pub fn register(
        &mut self,
        node: RelayNode,
    ) -> Result<(), &'static str> {
        if self.count >= MAX_RELAYS {
            return Err("directory full");
        }

        self.nodes[self.count] = Some(node);
        self.count += 1;

        Ok(())
    }
}
