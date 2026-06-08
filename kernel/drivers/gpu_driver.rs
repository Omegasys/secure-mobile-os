pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub address: usize,
}

pub struct GpuDriver {
    framebuffer: Option<Framebuffer>,
}

impl GpuDriver {
    pub const fn new() -> Self {
        Self {
            framebuffer: None,
        }
    }

    pub fn initialize(
        &mut self,
        width: u32,
        height: u32,
        address: usize,
    ) {
        self.framebuffer = Some(Framebuffer {
            width,
            height,
            address,
        });
    }

    pub fn clear_screen(&self) {
        if let Some(_fb) = &self.framebuffer {
            // Hardware framebuffer clear
        }
    }

    pub fn framebuffer(&self) -> Option<&Framebuffer> {
        self.framebuffer.as_ref()
    }
}
