pub struct CameraFrame {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}

pub struct CameraHAL {
    pub initialized: bool,
}

impl CameraHAL {
    pub fn new() -> Self {
        Self { initialized: false }
    }

    pub fn initialize(&mut self) -> Result<(), &'static str> {
        self.initialized = true;
        Ok(())
    }

    pub fn capture_frame(&self) -> Result<CameraFrame, &'static str> {
        if !self.initialized {
            return Err("camera not initialized");
        }

        Ok(CameraFrame {
            width: 1920,
            height: 1080,
            buffer: vec![0; 1920 * 1080 * 3],
        })
    }
}
