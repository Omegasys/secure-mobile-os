```rust
use sha2::{Digest, Sha256};
use std::fs;

pub fn sha256_file(path: &str) -> Result<String, String> {
    let data = fs::read(path)
        .map_err(|e| e.to_string())?;

    let mut hasher = Sha256::new();
    hasher.update(&data);

    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}

pub fn verify_hash(
    path: &str,
    expected_hash: &str,
) -> Result<bool, String> {
    let actual_hash = sha256_file(path)?;

    Ok(actual_hash.eq_ignore_ascii_case(expected_hash))
}

pub fn verify_image_exists(
    path: &str,
) -> Result<(), String> {
    if std::path::Path::new(path).exists() {
        Ok(())
    } else {
        Err(format!(
            "Image does not exist: {}",
            path
        ))
    }
}

pub fn print_image_info(path: &str) {
    match sha256_file(path) {
        Ok(hash) => {
            println!("File: {}", path);
            println!("SHA256: {}", hash);
        }
        Err(e) => {
            println!("Verification failed: {}", e);
        }
    }
}
```
