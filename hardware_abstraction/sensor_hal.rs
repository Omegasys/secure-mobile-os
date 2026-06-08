pub enum SensorType {
    Accelerometer,
    Gyroscope,
    Magnetometer,
    Proximity,
}

pub struct SensorData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct SensorHAL;

impl SensorHAL {
    pub fn new() -> Self {
        Self
    }

    pub fn read_sensor(&self, sensor: SensorType) -> Option<SensorData> {
        let _ = sensor;

        Some(SensorData {
            x: 0.0,
            y: 0.0,
            z: 9.8,
        })
    }
}
