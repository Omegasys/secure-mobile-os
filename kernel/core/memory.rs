pub struct MemoryManager {
    total_pages: usize,
    free_pages: usize,
}

impl MemoryManager {
    pub const fn new() -> Self {
        Self {
            total_pages: 0,
            free_pages: 0,
        }
    }

    pub fn initialize(&mut self) {
        self.total_pages = 1_048_576;
        self.free_pages = self.total_pages;
    }

    pub fn allocate_page(&mut self) -> Option<usize> {
        if self.free_pages == 0 {
            return None;
        }

        self.free_pages -= 1;

        Some(self.free_pages)
    }

    pub fn free_page(&mut self) {
        self.free_pages += 1;
    }
}
