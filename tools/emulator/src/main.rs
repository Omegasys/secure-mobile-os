```rust
mod vm;
mod cpu;
mod memory;

use vm::VirtualMachine;

fn main() {
    println!("Secure Mobile OS Emulator");

    let mut vm = VirtualMachine::new(
        "Secure Mobile OS".to_string(),
        4,
        4096,
    );

    vm.start();

    vm.cpu.print_info();
    vm.memory.print_info();

    println!("VM Status: {:?}", vm.state);
}
```
