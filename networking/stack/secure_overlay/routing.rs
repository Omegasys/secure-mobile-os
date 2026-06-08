pub const MAX_ROUTE_HOPS: usize = 16;

#[derive(Clone, Copy)]
pub struct RouteNode {
    pub node_id: u64,
}

pub struct SecureRoute {
    pub hop_count: usize,
    pub hops: [Option<RouteNode>; MAX_ROUTE_HOPS],
}

impl SecureRoute {
    pub const fn new() -> Self {
        Self {
            hop_count: 0,
            hops: [None; MAX_ROUTE_HOPS],
        }
    }

    pub fn add_hop(
        &mut self,
        node: RouteNode,
    ) -> Result<(), &'static str> {
        if self.hop_count >= MAX_ROUTE_HOPS {
            return Err("route full");
        }

        self.hops[self.hop_count] = Some(node);
        self.hop_count += 1;

        Ok(())
    }
}
