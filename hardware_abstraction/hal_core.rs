pub trait HalDevice {
    fn initialize(&mut self) -> Result<(), &'static str>;
    fn shutdown(&mut self);
    fn is_ready(&self) -> bool;
}

pub struct HalRegistry {
    pub camera_ready: bool,
    pub sensor_ready: bool,
    pub modem_ready: bool,
    pub crypto_ready: bool,
}

impl HalRegistry {
    pub const fn new() -> Self {
        Self {
            camera_ready: false,
            sensor_ready: false,
            modem_ready: false,
            crypto_ready: false,
        }
    }

    pub fn system_ready(&self) -> bool {
        self.camera_ready && self.sensor_ready && self.modem_ready
    }
}
