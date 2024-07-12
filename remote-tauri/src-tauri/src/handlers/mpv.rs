use actix_web::{get, HttpResponse, Responder};

use crate::clients;

#[tauri::command]
pub async fn handle_pause_cmd() {
    clients::mpv::toggle_pause().await;
}

#[get("/api/pause")]
pub async fn handle_pause() -> impl Responder {
    clients::mpv::toggle_pause().await;
    HttpResponse::Ok()
}

#[tauri::command]
pub async fn handle_volume_up_cmd() {
    println!("Volume up");
    clients::mpv::volume_up().await;
}

#[get("/api/volume_up")]
pub async fn handle_volume_up() -> impl Responder {
    clients::mpv::toggle_pause().await;
    HttpResponse::Ok()
}

#[tauri::command]
pub async fn handle_volume_down_cmd() {
    clients::mpv::volume_down().await;
}

#[get("/api/mpv/volume_down")]
pub async fn handle_volume_down() -> impl Responder {
    clients::mpv::toggle_pause().await;
    HttpResponse::Ok()
}
