```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id: u32,
    pub address: u64,
    pub enabled: bool,
    pub description: String,
}

pub struct BreakpointManager {
    breakpoints: HashMap<u32, Breakpoint>,
}

impl BreakpointManager {
    pub fn new() -> Self {
        Self {
            breakpoints: HashMap::new(),
        }
    }

    pub fn add_breakpoint(
        &mut self,
        id: u32,
        address: u64,
        description: String,
    ) {
        self.breakpoints.insert(
            id,
            Breakpoint {
                id,
                address,
                enabled: true,
                description,
            },
        );
    }

    pub fn remove_breakpoint(
        &mut self,
        id: u32,
    ) {
        self.breakpoints.remove(&id);
    }

    pub fn list_breakpoints(&self) {
        println!("Breakpoints");

        for bp in self.breakpoints.values() {
            println!(
                "#{} 0x{:X} enabled={} {}",
                bp.id,
                bp.address,
                bp.enabled,
                bp.description
            );
        }
    }

    pub fn toggle(
        &mut self,
        id: u32,
    ) {
        if let Some(bp) =
            self.breakpoints.get_mut(&id)
        {
            bp.enabled = !bp.enabled;
        }
    }
}
```
