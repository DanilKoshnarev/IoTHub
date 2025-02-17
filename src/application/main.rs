use actix_web::{web, App, HttpServer};
use iot_hub::domain::repositories::DeviceRepository;
use iot_hub::domain::services::DeviceService;
use iot_hub::domain::use_cases::ManageDevice;
use iot_hub::infrastructure::models::DeviceModel;
use iot_hub::infrastructure::controllers::DeviceController;

mod domain;
mod infrastructure;
mod application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let device_repository = DeviceModel::new();
    let device_service = DeviceService::new(device_repository);
    let manage_device = ManageDevice::new(device_service);
    let device_controller = DeviceController::new(manage_device);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(device_controller.clone()))
            .route("/devices", web::post().to(DeviceController::create_device))
            .route("/devices", web::get().to(DeviceController::get_all_devices))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
