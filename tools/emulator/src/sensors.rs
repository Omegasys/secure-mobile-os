```rust
#[derive(Debug)]
pub struct Accelerometer {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Gyroscope {
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
}

#[derive(Debug)]
pub struct Gps {
    pub latitude: f64,
    pub longitude: f64,
}

pub struct SensorManager {
    pub accelerometer: Accelerometer,
    pub gyroscope: Gyroscope,
    pub gps: Gps,
}

impl SensorManager {
    pub fn new() -> Self {
        Self {
            accelerometer: Accelerometer {
                x: 0.0,
                y: 0.0,
                z: 9.8,
            },
            gyroscope: Gyroscope {
                pitch: 0.0,
                roll: 0.0,
                yaw: 0.0,
            },
            gps: Gps {
                latitude: 0.0,
                longitude: 0.0,
            },
        }
    }

    pub fn update_gps(
        &mut self,
        latitude: f64,
        longitude: f64,
    ) {
        self.gps.latitude = latitude;
        self.gps.longitude = longitude;
    }

    pub fn print_info(&self) {
        println!("Sensors");

        println!(
            "Accelerometer: ({}, {}, {})",
            self.accelerometer.x,
            self.accelerometer.y,
            self.accelerometer.z
        );

        println!(
            "Gyroscope: ({}, {}, {})",
            self.gyroscope.pitch,
            self.gyroscope.roll,
            self.gyroscope.yaw
        );

        println!(
            "GPS: {}, {}",
            self.gps.latitude,
            self.gps.longitude
        );
    }
}
```
