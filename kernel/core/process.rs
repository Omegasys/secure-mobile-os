pub const MAX_PROCESSES: usize = 1024;

#[derive(Clone, Copy)]
pub enum ProcessState {
    Ready,
    Running,
    Sleeping,
    Terminated,
}

pub struct Process {
    pub pid: usize,
    pub state: ProcessState,
}

pub struct ProcessManager {
    next_pid: usize,
}

impl ProcessManager {
    pub const fn new() -> Self {
        Self {
            next_pid: 1,
        }
    }

    pub fn initialize(&mut self) {}

    pub fn create_process(&mut self) -> Process {
        let process = Process {
            pid: self.next_pid,
            state: ProcessState::Ready,
        };

        self.next_pid += 1;

        process
    }
}
