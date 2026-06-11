```rust
#[derive(Debug)]
pub struct FastbootDevice {
    pub serial: String,
    pub product: String,
    pub unlocked: bool,
}

pub fn detect_device() -> Option<FastbootDevice> {
    Some(FastbootDevice {
        serial: "TEST123456".to_string(),
        product: "SecurePhone".to_string(),
        unlocked: false,
    })
}

pub fn show_device_info() {
    match detect_device() {
        Some(device) => {
            println!("Fastboot Device Detected");
            println!("Serial: {}", device.serial);
            println!("Product: {}", device.product);
            println!("Unlocked: {}", device.unlocked);
        }
        None => {
            println!("No fastboot device found.");
        }
    }
}
```
