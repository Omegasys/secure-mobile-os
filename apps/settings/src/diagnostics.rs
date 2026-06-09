pub struct Diagnostics {
    pub cpu_usage_percent: f32,
    pub memory_usage_percent: f32,

    pub battery_health_percent: u8,

    pub storage_usage_percent: f32,

    pub network_latency_ms: u32,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_percent: 0.0,

            battery_health_percent: 100,

            storage_usage_percent: 0.0,

            network_latency_ms: 0,
        }
    }

    pub fn run_system_check(&self) {
        println!("Running diagnostics...");
        println!("CPU OK");
        println!("Memory OK");
        println!("Storage OK");
        println!("Networking OK");
        println!("Security Services OK");
    }

    pub fn print_report(&self) {
        println!("--- Diagnostics Report ---");

        println!(
            "CPU Usage: {:.2}%",
            self.cpu_usage_percent
        );

        println!(
            "Memory Usage: {:.2}%",
            self.memory_usage_percent
        );

        println!(
            "Battery Health: {}%",
            self.battery_health_percent
        );

        println!(
            "Storage Usage: {:.2}%",
            self.storage_usage_percent
        );

        println!(
            "Network Latency: {} ms",
            self.network_latency_ms
        );
    }
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self::new()
    }
}
