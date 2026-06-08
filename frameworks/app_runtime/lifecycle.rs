#[derive(Clone, Copy, PartialEq)]
pub enum AppState {
    Created,
    Running,
    Suspended,
    Terminated,
}

pub struct AppLifecycle {
    pub state: AppState,
    pub pid: u64,
}

impl AppLifecycle {
    pub fn new(pid: u64) -> Self {
        Self {
            state: AppState::Created,
            pid,
        }
    }

    pub fn start(&mut self) {
        self.state = AppState::Running;
    }

    pub fn suspend(&mut self) {
        self.state = AppState::Suspended;
    }

    pub fn resume(&mut self) {
        self.state = AppState::Running;
    }

    pub fn terminate(&mut self) {
        self.state = AppState::Terminated;
    }

    pub fn is_active(&self) -> bool {
        self.state == AppState::Running
    }
}
