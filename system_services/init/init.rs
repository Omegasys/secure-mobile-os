pub struct KernelInit;

impl KernelInit {
    pub fn boot_sequence() {
        Self::early_hardware_init();
        Self::memory_init();
        Self::start_service_manager();
    }

    fn early_hardware_init() {
        // CPU, timers, interrupt controller setup
    }

    fn memory_init() {
        // initialize heap, paging, allocator
    }

    fn start_service_manager() {
        // spawn service manager process
    }
}
