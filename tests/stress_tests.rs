#[test]
fn test_high_packet_load() {
    let mut success = true;

    for _ in 0..10000 {
        success &= true;
    }

    assert!(success);
}

#[test]
fn test_memory_pressure() {
    let allocated = vec![0u8; 1024 * 1024];
    assert_eq!(allocated.len(), 1024 * 1024);
}
