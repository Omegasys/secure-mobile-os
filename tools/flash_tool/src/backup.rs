```rust
use std::fs;
use std::path::Path;

pub fn create_backup(
    source_image: &str,
    backup_path: &str,
) -> Result<(), String> {

    if !Path::new(source_image).exists() {
        return Err(format!(
            "Source image not found: {}",
            source_image
        ));
    }

    fs::copy(source_image, backup_path)
        .map_err(|e| e.to_string())?;

    println!(
        "Backup created:\n  Source: {}\n  Backup: {}",
        source_image,
        backup_path
    );

    Ok(())
}

pub fn backup_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn list_backup_info(path: &str) -> Result<(), String> {
    let metadata =
        fs::metadata(path).map_err(|e| e.to_string())?;

    println!("Backup Information");
    println!("Path: {}", path);
    println!("Size: {} bytes", metadata.len());

    Ok(())
}
```
