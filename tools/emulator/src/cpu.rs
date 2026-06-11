```rust
#[derive(Debug)]
pub struct VirtualCpu {
    pub cores: u32,
    pub frequency_mhz: u32,
    pub architecture: String,
}

impl VirtualCpu {
    pub fn new(cores: u32) -> Self {
        Self {
            cores,
            frequency_mhz: 2400,
            architecture: "ARM64".to_string(),
        }
    }

    pub fn print_info(&self) {
        println!("CPU Information");
        println!("Architecture: {}", self.architecture);
        println!("Cores: {}", self.cores);
        println!(
            "Frequency: {} MHz",
            self.frequency_mhz
        );
    }

    pub fn execute_cycle(&self) {
        println!("Executing CPU cycle...");
    }
}
```
