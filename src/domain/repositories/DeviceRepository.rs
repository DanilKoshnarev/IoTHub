use crate::domain::entities::Device;

pub trait DeviceRepository {
    fn save(&self, device: Device);
    fn find_by_id(&self, id: &str) -> Option<Device>;
    fn find_all(&self) -> Vec<Device>;
    fn delete(&self, id: &str);
}
