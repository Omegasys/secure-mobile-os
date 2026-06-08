pub const BLOCK_SIZE: usize = 4096;

pub struct StorageDriver {
    initialized: bool,
}

impl StorageDriver {
    pub const fn new() -> Self {
        Self {
            initialized: false,
        }
    }

    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    pub fn read_block(
        &self,
        block: u64,
        buffer: &mut [u8],
    ) -> Result<(), &'static str> {
        let _ = block;
        let _ = buffer;

        Ok(())
    }

    pub fn write_block(
        &self,
        block: u64,
        buffer: &[u8],
    ) -> Result<(), &'static str> {
        let _ = block;
        let _ = buffer;

        Ok(())
    }
}
