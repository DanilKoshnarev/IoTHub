use crate::domain::entities::Device;
use crate::domain::repositories::DeviceRepository;

pub struct DeviceService<T: DeviceRepository> {
    repository: T,
}

impl<T: DeviceRepository> DeviceService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn add_device(&self, device: Device) {
        self.repository.save(device);
    }

    pub fn get_device(&self, id: &str) -> Option<Device> {
        self.repository.find_by_id(id)
    }

    pub fn get_all_devices(&self) -> Vec<Device> {
        self.repository.find_all()
    }

    pub fn delete_device(&self, id: &str) {
        self.repository.delete(id);
    }
}
