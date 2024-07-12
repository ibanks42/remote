use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::ClientOptions;

pub fn get_client() -> Result<tokio::net::windows::named_pipe::NamedPipeClient, std::io::Error> {
    let pipe_name = r"\\.\pipe\mpvpipe";
    ClientOptions::new().open(pipe_name)
}

pub async fn get_property(
    client: &mut tokio::net::windows::named_pipe::NamedPipeClient,
    property: &str,
) -> serde_json::Value {
    let msg = json!({
                    "command": ["get_property", property]
    });
    send_msg(client, msg).await;

    let mut buffer = [0; 1024];
    let n = client
        .read(&mut buffer)
        .await
        .map_err(|e| e.to_string())
        .unwrap();
    let response = std::str::from_utf8(&buffer[..n]).unwrap();
    let response_json: serde_json::Value = serde_json::from_str(response).unwrap();
    response_json["data"].clone()
}

pub async fn set_bool_property(
    client: &mut tokio::net::windows::named_pipe::NamedPipeClient,
    property: &str,
    value: bool,
) {
    let msg = json!({"command": ["set_property", property, value]});
    send_msg(client, msg).await;
}

pub async fn set_property(
    client: &mut tokio::net::windows::named_pipe::NamedPipeClient,
    property: &str,
    value: &str,
) {
    let msg = json!({"command": ["set_property_string", property, value]});
    send_msg(client, msg).await;
}

pub async fn send_msg(
    client: &mut tokio::net::windows::named_pipe::NamedPipeClient,
    msg: serde_json::Value,
) {
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
