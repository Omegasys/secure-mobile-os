```rust
use crate::cpu::VirtualCpu;
use crate::memory::VirtualMemory;

#[derive(Debug)]
pub enum VmState {
    Stopped,
    Running,
    Paused,
}

pub struct VirtualMachine {
    pub name: String,
    pub cpu: VirtualCpu,
    pub memory: VirtualMemory,
    pub state: VmState,
}

impl VirtualMachine {
    pub fn new(
        name: String,
        cpu_cores: u32,
        memory_mb: u64,
    ) -> Self {
        Self {
            name,
            cpu: VirtualCpu::new(cpu_cores),
            memory: VirtualMemory::new(memory_mb),
            state: VmState::Stopped,
        }
    }

    pub fn start(&mut self) {
        self.state = VmState::Running;

        println!(
            "Starting VM '{}'",
            self.name
        );
    }

    pub fn stop(&mut self) {
        self.state = VmState::Stopped;

        println!(
            "Stopping VM '{}'",
            self.name
        );
    }

    pub fn pause(&mut self) {
        self.state = VmState::Paused;

        println!(
            "Pausing VM '{}'",
            self.name
        );
    }
}
```
