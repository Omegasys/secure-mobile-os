```rust
#[derive(Debug)]
pub struct IpcChannel {
    pub id: u32,
    pub source_pid: u32,
    pub destination_pid: u32,
    pub messages_pending: usize,
}

pub fn get_channels() -> Vec<IpcChannel> {
    vec![
        IpcChannel {
            id: 1,
            source_pid: 1,
            destination_pid: 10,
            messages_pending: 0,
        },
        IpcChannel {
            id: 2,
            source_pid: 10,
            destination_pid: 25,
            messages_pending: 3,
        },
        IpcChannel {
            id: 3,
            source_pid: 25,
            destination_pid: 1,
            messages_pending: 1,
        },
    ]
}

pub fn show_channels() {
    println!("IPC Channels");
    println!("------------");

    for channel in get_channels() {
        println!(
            "ID={} {} -> {} Pending={}",
            channel.id,
            channel.source_pid,
            channel.destination_pid,
            channel.messages_pending
        );
    }
}

pub fn find_channel(id: u32) -> Option<IpcChannel> {
    get_channels()
        .into_iter()
        .find(|c| c.id == id)
}
```
