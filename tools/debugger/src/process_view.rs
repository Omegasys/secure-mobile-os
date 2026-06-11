```rust
#[derive(Debug, Clone)]
pub enum ProcessState {
    Running,
    Sleeping,
    Waiting,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory_kb: u64,
    pub state: ProcessState,
}

pub fn get_processes() -> Vec<ProcessInfo> {
    vec![
        ProcessInfo {
            pid: 1,
            name: "init".to_string(),
            memory_kb: 4096,
            state: ProcessState::Running,
        },
        ProcessInfo {
            pid: 10,
            name: "securityd".to_string(),
            memory_kb: 8192,
            state: ProcessState::Running,
        },
        ProcessInfo {
            pid: 25,
            name: "networkd".to_string(),
            memory_kb: 6144,
            state: ProcessState::Sleeping,
        },
    ]
}

pub fn show_processes() {
    println!("Process List");
    println!("------------");

    for process in get_processes() {
        println!(
            "PID={} Name={} Memory={}KB State={:?}",
            process.pid,
            process.name,
            process.memory_kb,
            process.state
        );
    }
}

pub fn find_process(pid: u32) -> Option<ProcessInfo> {
    get_processes()
        .into_iter()
        .find(|p| p.pid == pid)
}
```
