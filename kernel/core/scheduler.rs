pub struct Scheduler {
    current_pid: usize,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            current_pid: 0,
        }
    }

    pub fn initialize(&mut self) {
        self.current_pid = 0;
    }

    pub fn schedule(&mut self) {
        self.current_pid += 1;

        if self.current_pid > 1024 {
            self.current_pid = 0;
        }

        self.context_switch();
    }

    fn context_switch(&self) {
        // Architecture-specific switch logic
    }
}
