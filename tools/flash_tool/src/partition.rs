```rust
#[derive(Debug, Clone)]
pub struct Partition {
    pub name: String,
    pub size_bytes: u64,
    pub writable: bool,
}

pub fn get_default_partitions() -> Vec<Partition> {
    vec![
        Partition {
            name: "boot".to_string(),
            size_bytes: 64 * 1024 * 1024,
            writable: true,
        },
        Partition {
            name: "system".to_string(),
            size_bytes: 4 * 1024 * 1024 * 1024,
            writable: true,
        },
        Partition {
            name: "vendor".to_string(),
            size_bytes: 1024 * 1024 * 1024,
            writable: true,
        },
        Partition {
            name: "userdata".to_string(),
            size_bytes: 32 * 1024 * 1024 * 1024,
            writable: true,
        },
    ]
}

pub fn find_partition(name: &str) -> Option<Partition> {
    get_default_partitions()
        .into_iter()
        .find(|p| p.name == name)
}

pub fn print_partition_table() {
    println!("Partition Table");

    for partition in get_default_partitions() {
        println!(
            "{} | {} bytes | writable={}",
            partition.name,
            partition.size_bytes,
            partition.writable
        );
    }
}
```
