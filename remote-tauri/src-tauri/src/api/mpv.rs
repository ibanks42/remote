use actix_web::{get, HttpResponse, Responder};

use crate::clients;

#[tauri::command]
pub async fn handle_pause_cmd() {
    clients::mpv::toggle_pause().await;
}

#[get("/api/mpv/pause")]
pub async fn handle_pause_api() -> impl Responder {
    println!("toggling pause api called");
    clients::mpv::toggle_pause().await;
    HttpResponse::Ok()
}

#[tauri::command]
pub async fn handle_volume_up_cmd() {
    println!("Volume up");
    clients::mpv::volume_up().await;
}

#[get("/api/mpv/volume-up")]
pub async fn handle_volume_up_api() -> impl Responder {
    clients::mpv::volume_up().await;
    HttpResponse::Ok()
}

#[tauri::command]
pub async fn handle_volume_down_cmd() {
    clients::mpv::volume_down().await;
}

#[get("/api/mpv/volume-down")]
pub async fn handle_volume_down_api() -> impl Responder {
    clients::mpv::volume_down().await;
    HttpResponse::Ok()
}

#[get("/api/mpv/status")]
pub async fn handle_status_api() -> impl Responder {
    HttpResponse::Ok().json(clients::mpv::get_status().await.unwrap())
}
