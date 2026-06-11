```rust
#[derive(Debug)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
    pub permissions: String,
    pub description: String,
}

pub fn get_memory_regions() -> Vec<MemoryRegion> {
    vec![
        MemoryRegion {
            start: 0x1000,
            end: 0x7FFF,
            permissions: "r-x".to_string(),
            description: "Kernel Code".to_string(),
        },
        MemoryRegion {
            start: 0x8000,
            end: 0xFFFF,
            permissions: "rw-".to_string(),
            description: "Kernel Data".to_string(),
        },
        MemoryRegion {
            start: 0x10000,
            end: 0x1FFFF,
            permissions: "rw-".to_string(),
            description: "User Heap".to_string(),
        },
    ]
}

pub fn show_memory_map() {
    println!("Memory Map");
    println!("----------");

    for region in get_memory_regions() {
        println!(
            "0x{:X}-0x{:X} {} {}",
            region.start,
            region.end,
            region.permissions,
            region.description
        );
    }
}

pub fn total_memory_used() -> u64 {
    get_memory_regions()
        .iter()
        .map(|r| r.end - r.start)
        .sum()
}
```
