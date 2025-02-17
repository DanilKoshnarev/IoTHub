pub struct SensorData {
    pub id: String,
    pub device_id: String,
    pub data: String,
    pub timestamp: String,
}

impl SensorData {
    pub fn new(id: String, device_id: String, data: String, timestamp: String) -> Self {
        Self { id, device_id, data, timestamp }
    }
}
