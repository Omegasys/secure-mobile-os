```rust
#[derive(Debug)]
pub struct VirtualStorage {
    pub total_mb: u64,
    pub used_mb: u64,
}

impl VirtualStorage {
    pub fn new(total_mb: u64) -> Self {
        Self {
            total_mb,
            used_mb: 0,
        }
    }

    pub fn write(
        &mut self,
        size_mb: u64,
    ) -> Result<(), String> {

        if self.used_mb + size_mb > self.total_mb {
            return Err(
                "Insufficient storage space".to_string()
            );
        }

        self.used_mb += size_mb;
        Ok(())
    }

    pub fn delete(&mut self, size_mb: u64) {
        self.used_mb =
            self.used_mb.saturating_sub(size_mb);
    }

    pub fn free_space(&self) -> u64 {
        self.total_mb - self.used_mb
    }

    pub fn print_info(&self) {
        println!("Storage");
        println!("Total: {} MB", self.total_mb);
        println!("Used: {} MB", self.used_mb);
        println!("Free: {} MB", self.free_space());
    }
}
```
