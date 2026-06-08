pub struct StorageAPI {
    pub allowed_read: bool,
    pub allowed_write: bool,
}

impl StorageAPI {
    pub fn new() -> Self {
        Self {
            allowed_read: false,
            allowed_write: false,
        }
    }

    pub fn grant_read(&mut self) {
        self.allowed_read = true;
    }

    pub fn grant_write(&mut self) {
        self.allowed_write = true;
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, &'static str> {
        if !self.allowed_read {
            return Err("read permission denied");
        }

        // placeholder filesystem access
        Ok(path.as_bytes().to_vec())
    }

    pub fn write_file(&self, path: &str, data: &[u8]) -> Result<(), &'static str> {
        if !self.allowed_write {
            return Err("write permission denied");
        }

        let _ = (path, data);
        Ok(())
    }
}
