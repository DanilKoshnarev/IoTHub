use crate::domain::entities::Device;
use crate::domain::services::DeviceService;

pub struct ManageDevice {
    device_service: DeviceService,
}

impl ManageDevice {
    pub fn new(device_service: DeviceService) -> Self {
        Self { device_service }
    }

    pub fn create_device(&self, id: String, name: String, status: String, ip_address: String) {
        let device = Device::new(id, name, status, ip_address);
        self.device_service.add_device(device);
    }

    pub fn view_device(&self, id: &str) -> Option<Device> {
        self.device_service.get_device(id)
    }

    pub fn view_all_devices(&self) -> Vec<Device> {
        self.device_service.get_all_devices()
    }

    pub fn remove_device(&self, id: &str) {
        self.device_service.delete_device(id);
    }
}
