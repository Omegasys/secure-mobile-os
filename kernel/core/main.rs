#![no_std]
#![no_main]

mod scheduler;
mod memory;
mod process;
mod syscall;

use scheduler::Scheduler;
use memory::MemoryManager;
use process::ProcessManager;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut memory = MemoryManager::new();
    memory.initialize();

    let mut process_manager = ProcessManager::new();
    process_manager.initialize();

    let mut scheduler = Scheduler::new();
    scheduler.initialize();

    loop {
        scheduler.schedule();
    }
}
