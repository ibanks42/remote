use crate::settings::load_settings;
use serde_json::{from_str, json, Value};
use std::str::from_utf8;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(windows)]
use tokio::net::windows::named_pipe::{ClientOptions, NamedPipeClient};
#[cfg(unix)]
use tokio::net::UnixStream;

#[cfg(windows)]
pub fn get_client() -> Result<NamedPipeClient, std::io::Error> {
    ClientOptions::new().open(load_settings().mpv.pipe.as_str())
}

#[cfg(windows)]
pub async fn get_property(client: &mut NamedPipeClient, property: &str) -> serde_json::Value {
    let msg = json!({"command": ["get_property", property]});
    send_msg(client, msg).await;

    let mut buffer = [0; 1024];
    let n = client
        .read(&mut buffer)
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    let response = from_utf8(&buffer[..n]).unwrap();
    let response_json: Value = from_str(response).unwrap();

    response_json["data"].clone()
}
#[cfg(windows)]
pub async fn set_bool_property(client: &mut NamedPipeClient, property: &str, value: bool) {
    let msg = json!({"command": ["set_property", property, value]});
    send_msg(client, msg).await;
}
#[cfg(windows)]
pub async fn set_property(client: &mut NamedPipeClient, property: &str, value: &str) {
    let msg = json!({"command": ["set_property_string", property, value]});
    send_msg(client, msg).await;
}
#[cfg(windows)]
pub async fn send_msg(client: &mut NamedPipeClient, msg: Value) {
    client
        .write_all(msg.to_string().as_bytes())
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    client
        .write_all(b"\n")
        .await
        .map_err(|e| e.to_string())
        .unwrap();
}

#[cfg(unix)]
pub async fn send_msg(client: &mut UnixStream, msg: Value) {
    client
        .write_all(msg.to_string().as_bytes())
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    client
        .write_all(b"\n")
        .await
        .map_err(|e| e.to_string())
        .unwrap();
}

#[cfg(unix)]
pub async fn get_client() -> Result<UnixStream, std::io::Error> {
    UnixStream::connect(load_settings().mpv.pipe.as_str()).await
}

#[cfg(unix)]
pub async fn get_property(client: &mut UnixStream, property: &str) -> Value {
    let msg = json!({"command": ["get_property", property]});
    send_msg(client, msg).await;

    let mut buffer = [0; 1024];
    let n = client
        .read(&mut buffer)
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    let response = from_utf8(&buffer[..n]).unwrap();
    let response_json: Value = from_str(response).unwrap();

    response_json["data"].clone()
}

#[cfg(unix)]
pub async fn set_bool_property(client: &mut UnixStream, property: &str, value: bool) {
    let msg = json!({"command": ["set_property", property, value]});
    send_msg(client, msg).await;
}

#[cfg(unix)]
pub async fn set_property(client: &mut UnixStream, property: &str, value: &str) {
    let msg = json!({"command": ["set_property_string", property, value]});
    send_msg(client, msg).await;
}
