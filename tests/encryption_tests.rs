#[test]
fn test_encrypt_decrypt_cycle() {
    let original = b"hello";
    let mut data = original.clone();

    let key = [1u8; 16];

    for b in &mut data {
        *b ^= key[0];
    }

    for b in &mut data {
        *b ^= key[0];
    }

    assert_eq!(&data, original);
}
