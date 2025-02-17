use actix_web::{web, HttpResponse};
use crate::domain::use_cases::ManageDevice;
use serde::Deserialize;

pub struct DeviceController {
    manage_device: ManageDevice,
}

#[derive(Deserialize)]
pub struct CreateDeviceRequest {
    pub id: String,
    pub name: String,
    pub status: String,
    pub ip_address: String,
}

impl DeviceController {
    pub fn new(manage_device: ManageDevice) -> Self {
        Self { manage_device }
    }

    pub async fn create_device(
        manage_device: web::Data<Self>,
        request: web::Json<CreateDeviceRequest>,
    ) -> HttpResponse {
        manage_device.manage_device.create_device(
            request.id.clone(),
            request.name.clone(),
            request.status.clone(),
            request.ip_address.clone(),
        );
        HttpResponse::Ok().finish()
    }

    pub async fn get_all_devices(
        manage_device: web::Data<Self>,
    ) -> HttpResponse {
        let devices = manage_device.manage_device.view_all_devices();
        HttpResponse::Ok().json(devices)
    }
}
