```rust
#[derive(Debug)]
pub enum ChargingState {
    Charging,
    Discharging,
    Full,
}

pub struct Battery {
    pub capacity_percent: u8,
    pub temperature_c: f32,
    pub charging_state: ChargingState,
}

impl Battery {
    pub fn new() -> Self {
        Self {
            capacity_percent: 100,
            temperature_c: 25.0,
            charging_state: ChargingState::Full,
        }
    }

    pub fn charge(&mut self, amount: u8) {
        self.capacity_percent =
            self.capacity_percent.saturating_add(amount);

        if self.capacity_percent > 100 {
            self.capacity_percent = 100;
        }

        self.charging_state = if self.capacity_percent == 100 {
            ChargingState::Full
        } else {
            ChargingState::Charging
        };
    }

    pub fn drain(&mut self, amount: u8) {
        self.capacity_percent =
            self.capacity_percent.saturating_sub(amount);

        self.charging_state =
            ChargingState::Discharging;
    }

    pub fn print_info(&self) {
        println!("Battery");
        println!(
            "Capacity: {}%",
            self.capacity_percent
        );
        println!(
            "Temperature: {:.1}°C",
            self.temperature_c
        );
        println!(
            "State: {:?}",
            self.charging_state
        );
    }
}
```
