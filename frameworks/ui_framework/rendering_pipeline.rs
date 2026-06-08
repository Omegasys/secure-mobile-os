pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub struct RenderingPipeline {
    pub vsync_enabled: bool,
}

impl RenderingPipeline {
    pub fn new() -> Self {
        Self {
            vsync_enabled: true,
        }
    }

    pub fn draw_frame(&self, buffer: &mut FrameBuffer) {
        // Placeholder rendering logic
        let _ = buffer;
    }

    pub fn enable_vsync(&mut self) {
        self.vsync_enabled = true;
    }

    pub fn disable_vsync(&mut self) {
        self.vsync_enabled = false;
    }
}
