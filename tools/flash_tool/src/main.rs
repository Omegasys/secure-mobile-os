```rust
mod flash;
mod fastboot;
mod recovery;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "secure-flash-tool")]
#[command(version = "0.1")]
#[command(about = "Secure Mobile OS Flash Utility")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Flash {
        image: String,
        partition: String,
    },
    FastbootInfo,
    RecoveryInfo,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Flash { image, partition } => {
            match flash::flash_image(&image, &partition) {
                Ok(_) => println!("Flash operation completed."),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Commands::FastbootInfo => {
            fastboot::show_device_info();
        }

        Commands::RecoveryInfo => {
            recovery::show_recovery_status();
        }
    }
}
```
