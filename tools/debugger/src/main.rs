```rust
mod process_view;
mod memory_view;
mod ipc_view;

fn main() {
    println!("Secure Mobile OS Debugger");
    println!("=========================");

    process_view::show_processes();
    println!();

    memory_view::show_memory_map();
    println!();

    ipc_view::show_channels();
}
```
