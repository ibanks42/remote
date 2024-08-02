use salvo::{handler, writing::Text, Depot, Request, Response};

use crate::clients;

#[tauri::command]
pub async fn handle_pause_cmd() {
    tracing::debug!("[Tauri] Toggling pause");
    clients::mpv::toggle_pause().await;
}

#[handler]
pub async fn handle_pause_api(_req: &mut Request, res: &mut Response, _depot: &mut Depot) {
    tracing::debug!("[API: MPV] Toggling pause");
    clients::mpv::toggle_pause().await;
    res.render(Text::Plain("ok"))
}

#[handler]
pub async fn handle_set_subtitle(req: &mut Request, res: &mut Response, _depot: &mut Depot) {
    let id = req.query::<i64>("id").expect("No subtitle ID provided");
    tracing::debug!("[API: MPV] Setting subtitle to {}", id);

    clients::mpv::set_subtitle(id).await;
    res.render(Text::Plain("ok"))
}

#[tauri::command]
pub async fn handle_volume_up_cmd() {
    tracing::debug!("[Tauri] Volume up");
    clients::mpv::volume_up().await;
}

#[handler]
pub async fn handle_skip_backward_api(_req: &mut Request, res: &mut Response, _depot: &mut Depot) {
    tracing::debug!("[API: MPV] Skip forward");
    clients::mpv::skip_backward().await;
    res.render(Text::Plain("ok"))
}

#[handler]
pub async fn handle_skip_forward_api(_req: &mut Request, res: &mut Response, _depot: &mut Depot) {
    tracing::debug!("[API: MPV] Skip forward");
    clients::mpv::skip_forward().await;
    res.render(Text::Plain("ok"))
}

#[handler]
pub async fn handle_set_volume_api(req: &mut Request, res: &mut Response, _depot: &mut Depot) {
    let volume = req.query::<i16>("volume").expect("No volume provided");
    tracing::debug!("[API: MPV] Setting volume to {}", volume);

    clients::mpv::set_volume(volume).await;
    res.render(Text::Plain("ok"))
}

#[handler]
pub async fn handle_volume_up_api(_req: &mut Request, res: &mut Response, _depot: &mut Depot) {
    tracing::debug!("[API: MPV] Volume up");
    clients::mpv::volume_up().await;
    res.render(Text::Plain("ok"))
}

#[tauri::command]
pub async fn handle_volume_down_cmd() {
    tracing::debug!("[Tauri] Volume down");
    clients::mpv::volume_down().await;
}

#[handler]
pub async fn handle_volume_down_api(_req: &mut Request, res: &mut Response, _depot: &mut Depot) {
    tracing::debug!("[API: MPV] Volume down");
    clients::mpv::volume_down().await;
    res.render(Text::Plain("ok"))
}

#[handler]
pub async fn handle_status_api(_req: &mut Request, res: &mut Response, _depot: &mut Depot) {
    tracing::debug!("[API: MPV] Get MPV status");
    let status = clients::mpv::get_status().await.unwrap();
    let response = serde_json::to_string(&status).expect("Failed to serialize status");
    res.render(Text::Json(response))
}
