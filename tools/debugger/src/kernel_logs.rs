```rust
#[derive(Debug, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone)]
pub struct KernelLogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

pub fn get_logs() -> Vec<KernelLogEntry> {
    vec![
        KernelLogEntry {
            timestamp: "00:00:01".to_string(),
            level: LogLevel::Info,
            message: "Kernel initialized".to_string(),
        },
        KernelLogEntry {
            timestamp: "00:00:02".to_string(),
            level: LogLevel::Info,
            message: "Memory manager online".to_string(),
        },
        KernelLogEntry {
            timestamp: "00:00:03".to_string(),
            level: LogLevel::Warning,
            message: "Low entropy pool".to_string(),
        },
    ]
}

pub fn show_logs() {
    println!("Kernel Logs");
    println!("-----------");

    for log in get_logs() {
        println!(
            "[{}] {:?}: {}",
            log.timestamp,
            log.level,
            log.message
        );
    }
}

pub fn filter_logs(
    level: LogLevel,
) -> Vec<KernelLogEntry> {
    get_logs()
        .into_iter()
        .filter(|l| {
            std::mem::discriminant(&l.level)
                == std::mem::discriminant(&level)
        })
        .collect()
}
```
