```rust
#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: String,
    pub product: String,
}

pub fn enumerate_devices() -> Vec<UsbDevice> {
    vec![
        UsbDevice {
            vendor_id: 0x18D1,
            product_id: 0x4EE0,
            manufacturer: "Secure Mobile OS".to_string(),
            product: "Development Device".to_string(),
        }
    ]
}

pub fn print_devices() {
    let devices = enumerate_devices();

    if devices.is_empty() {
        println!("No USB devices found.");
        return;
    }

    println!("Detected USB Devices");

    for device in devices {
        println!(
            "{:04X}:{:04X} {} {}",
            device.vendor_id,
            device.product_id,
            device.manufacturer,
            device.product
        );
    }
}

pub fn find_device(
    vendor_id: u16,
    product_id: u16,
) -> Option<UsbDevice> {
    enumerate_devices()
        .into_iter()
        .find(|d| {
            d.vendor_id == vendor_id
                && d.product_id == product_id
        })
}
```
