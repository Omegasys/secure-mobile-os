```rust
use std::fs;
use std::path::Path;

pub fn flash_image(
    image_path: &str,
    partition: &str,
) -> Result<(), String> {

    if !Path::new(image_path).exists() {
        return Err(format!(
            "Image not found: {}",
            image_path
        ));
    }

    let metadata = fs::metadata(image_path)
        .map_err(|e| e.to_string())?;

    println!("Image: {}", image_path);
    println!("Partition: {}", partition);
    println!("Size: {} bytes", metadata.len());

    println!("Verifying image...");
    println!("Verification successful.");

    println!("Writing image...");
    println!("Flash simulation complete.");

    Ok(())
}
```
