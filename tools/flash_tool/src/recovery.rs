#[derive(Debug)]
pub struct RecoveryStatus {
    pub active: bool,
    pub version: String,
    pub verified_boot: bool,
}

pub fn get_recovery_status() -> RecoveryStatus {
    RecoveryStatus {
        active: true,
        version: "0.1.0".to_string(),
        verified_boot: true,
    }
}

pub fn show_recovery_status() {
    let status = get_recovery_status();

    println!("Recovery Environment");
    println!("Version: {}", status.version);
    println!("Active: {}", status.active);
    println!(
        "Verified Boot: {}",
        status.verified_boot
    );
}
