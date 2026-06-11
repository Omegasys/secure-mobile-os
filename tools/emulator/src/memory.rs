#[derive(Debug)]
pub struct VirtualMemory {
    pub total_mb: u64,
    pub used_mb: u64,
}

impl VirtualMemory {
    pub fn new(total_mb: u64) -> Self {
        Self {
            total_mb,
            used_mb: 0,
        }
    }

    pub fn allocate(
        &mut self,
        amount_mb: u64,
    ) -> Result<(), String> {

        if self.used_mb + amount_mb > self.total_mb {
            return Err(
                "Out of virtual memory".to_string()
            );
        }

        self.used_mb += amount_mb;

        Ok(())
    }

    pub fn free(&mut self, amount_mb: u64) {
        self.used_mb =
            self.used_mb.saturating_sub(amount_mb);
    }

    pub fn print_info(&self) {
        println!("Memory Information");
        println!("Total: {} MB", self.total_mb);
        println!("Used: {} MB", self.used_mb);
        println!(
            "Free: {} MB",
            self.total_mb - self.used_mb
        );
    }
}
