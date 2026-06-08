pub enum NetworkState {
    Off,
    Searching,
    Connected,
}

pub struct ModemHAL {
    state: NetworkState,
}

impl ModemHAL {
    pub fn new() -> Self {
        Self {
            state: NetworkState::Off,
        }
    }

    pub fn power_on(&mut self) {
        self.state = NetworkState::Searching;
    }

    pub fn connect(&mut self) {
        self.state = NetworkState::Connected;
    }

    pub fn disconnect(&mut self) {
        self.state = NetworkState::Off;
    }

    pub fn state(&self) -> NetworkState {
        self.state
    }
}
