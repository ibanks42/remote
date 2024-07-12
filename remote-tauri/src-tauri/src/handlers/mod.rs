pub mod example;
pub mod mpv;

use std::sync::Mutex;

use actix_web::{middleware, web, App, HttpServer};
use tauri::AppHandle;

#[allow(dead_code)]
struct TauriAppState {
    app: Mutex<AppHandle>,
}

#[actix_web::main]
pub async fn init(app: AppHandle) -> std::io::Result<()> {
    println!("Server started");
    let tauri_app = web::Data::new(TauriAppState {
        app: Mutex::new(app),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(tauri_app.clone())
            .wrap(middleware::Logger::default())
            .service(example::handle)
            .service(mpv::handle_pause_api)
            .service(mpv::handle_volume_up_api)
            .service(mpv::handle_volume_down_api)
    })
    .bind(("0.0.0.0", 6969))?
    .run()
    .await
}
