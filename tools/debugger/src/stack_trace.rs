```rust
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub frame_number: usize,
    pub instruction_pointer: u64,
    pub function_name: String,
}

pub fn get_stack_trace(
    pid: u32,
) -> Vec<StackFrame> {
    vec![
        StackFrame {
            frame_number: 0,
            instruction_pointer: 0x1000,
            function_name: "main".to_string(),
        },
        StackFrame {
            frame_number: 1,
            instruction_pointer: 0x1100,
            function_name: "scheduler_tick".to_string(),
        },
        StackFrame {
            frame_number: 2,
            instruction_pointer: 0x1200,
            function_name: "context_switch".to_string(),
        },
    ]
}

pub fn show_stack_trace(pid: u32) {
    println!(
        "Stack Trace for PID {}",
        pid
    );

    for frame in get_stack_trace(pid) {
        println!(
            "#{} 0x{:X} {}",
            frame.frame_number,
            frame.instruction_pointer,
            frame.function_name
        );
    }
}
```
