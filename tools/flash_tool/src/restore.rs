```rust
use std::fs;
use std::path::Path;

pub fn restore_backup(
    backup_path: &str,
    destination_image: &str,
) -> Result<(), String> {

    if !Path::new(backup_path).exists() {
        return Err(format!(
            "Backup not found: {}",
            backup_path
        ));
    }

    fs::copy(backup_path, destination_image)
        .map_err(|e| e.to_string())?;

    println!(
        "Restore completed:\n  Backup: {}\n  Destination: {}",
        backup_path,
        destination_image
    );

    Ok(())
}

pub fn verify_restore_target(
    destination: &str,
) -> Result<(), String> {

    if Path::new(destination).exists() {
        println!(
            "Warning: destination already exists."
        );
    }

    Ok(())
}

pub fn restore_info(
    backup_path: &str,
) -> Result<(), String> {

    let metadata =
        fs::metadata(backup_path).map_err(|e| e.to_string())?;

    println!("Restore Source");
    println!("Path: {}", backup_path);
    println!("Size: {} bytes", metadata.len());

    Ok(())
}
```
