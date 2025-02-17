pub struct Device {
    pub id: String,
    pub name: String,
    pub status: String,
    pub ip_address: String,
}

impl Device {
    pub fn new(id: String, name: String, status: String, ip_address: String) -> Self {
        Self { id, name, status, ip_address }
    }
}
