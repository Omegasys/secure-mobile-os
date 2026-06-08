pub const MAX_NODES: usize = 128;

pub struct ServiceGraph {
    dependencies: [[bool; MAX_NODES]; MAX_NODES],
}

impl ServiceGraph {
    pub const fn new() -> Self {
        Self {
            dependencies: [[false; MAX_NODES]; MAX_NODES],
        }
    }

    pub fn add_dependency(&mut self, from: usize, to: usize) {
        if from < MAX_NODES && to < MAX_NODES {
            self.dependencies[from][to] = true;
        }
    }

    pub fn depends_on(&self, from: usize, to: usize) -> bool {
        self.dependencies[from][to]
    }
}
