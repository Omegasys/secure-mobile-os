#[derive(Clone, Copy)]
pub enum RadioState {
    Off,
    Searching,
    Connected,
}

pub struct ModemDriver {
    state: RadioState,
}

impl ModemDriver {
    pub const fn new() -> Self {
        Self {
            state: RadioState::Off,
        }
    }

    pub fn initialize(&mut self) {
        self.state = RadioState::Searching;
    }

    pub fn connect(&mut self) {
        self.state = RadioState::Connected;
    }

    pub fn disconnect(&mut self) {
        self.state = RadioState::Off;
    }

    pub fn signal_strength(&self) -> u8 {
        100
    }

    pub fn state(&self) -> RadioState {
        self.state
    }
}
