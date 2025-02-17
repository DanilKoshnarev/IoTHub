use crate::domain::entities::Device;
use crate::domain::repositories::DeviceRepository;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct DeviceModel {
    devices: Mutex<HashMap<String, Device>>,
}

impl DeviceModel {
    pub fn new() -> Self {
        Self {
            devices: Mutex::new(HashMap::new()),
        }
    }
}

impl DeviceRepository for DeviceModel {
    fn save(&self, device: Device) {
        let mut devices = self.devices.lock().unwrap();
        devices.insert(device.id.clone(), device);
    }

    fn find_by_id(&self, id: &str) -> Option<Device> {
        let devices = self.devices.lock().unwrap();
        devices.get(id).cloned()
    }

    fn find_all(&self) -> Vec<Device> {
        let devices = self.devices.lock().unwrap();
        devices.values().cloned().collect()
    }

    fn delete(&self, id: &str) {
        let mut devices = self.devices.lock().unwrap();
        devices.remove(id);
    }
}
