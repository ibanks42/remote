use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

mod pipe;

pub async fn toggle_pause() {
    println!("toggling pause");
    #[cfg(windows)]
    let client = pipe::get_client();

    #[cfg(unix)]
    let client = pipe::get_client().await;

    if client.is_err() {
        println!("Error getting client: {:?}", client.err().unwrap());
        return;
    }

    let mut client = client.unwrap();

    let is_paused = pipe::get_property(&mut client, "pause")
        .await
        .as_bool()
        .unwrap();

    println!("is_paused: {}", !is_paused);

    pipe::set_bool_property(&mut client, "pause", !is_paused).await;
}

pub async fn volume_up() {
    #[cfg(windows)]
    let client = pipe::get_client();
    #[cfg(unix)]
    let client = pipe::get_client().await;

    if client.is_err() {
        println!("Error getting client: {:?}", client.err().unwrap());
        return;
    }

    let mut client = client.unwrap();

    let volume = pipe::get_property(&mut client, "volume").await.as_f64();
    if volume.is_none() {
        println!("Error getting volume");
        return;
    }
    let volume = volume.unwrap().add(2.0f64);

    pipe::set_property(&mut client, "volume", volume.to_string().as_str()).await;
}

pub async fn volume_down() {
    #[cfg(windows)]
    let client = pipe::get_client();
    #[cfg(unix)]
    let client = pipe::get_client().await;

    if client.is_err() {
        println!("Error getting client: {:?}", client.err().unwrap());
        return;
    }
    let mut client = client.unwrap();

    let volume = pipe::get_property(&mut client, "volume").await.as_f64();
    if volume.is_none() {
        println!("Error getting volume");
        return;
    }
    let volume = volume.unwrap().sub(2.0f64);

    pipe::set_property(&mut client, "volume", volume.to_string().as_str()).await;
}

pub async fn get_status() -> Result<Status, Box<dyn std::error::Error>> {
    #[cfg(windows)]
    let client = pipe::get_client();
    #[cfg(unix)]
    let client = pipe::get_client().await;

    if client.is_err() {
        return Err(format!("Error getting client: {:?}", client.err().unwrap()).into());
    }

    let mut client = client.unwrap();

    let paused = pipe::get_property(&mut client, "pause")
        .await
        .as_bool()
        .unwrap_or(true);

    let volume = pipe::get_property(&mut client, "volume")
        .await
        .as_f64()
        .unwrap_or(0.0);

    let position = pipe::get_property(&mut client, "time-pos")
        .await
        .as_f64()
        .unwrap_or(0.0);

    let length = pipe::get_property(&mut client, "duration")
        .await
        .as_f64()
        .unwrap_or(0.0);

    let title = pipe::get_property(&mut client, "media-title")
        .await
        .to_string();

    let file = pipe::get_property(&mut client, "filename")
        .await
        .to_string();

    println!(
        "paused: {}, volume: {}, position: {}, length: {}, title: {}, file: {}",
        paused, volume, position, length, title, file
    );

    Ok(Status {
        paused,
        volume,
        position,
        length,
        title,
        file,
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub paused: bool,
    pub volume: f64,
    pub position: f64,
    pub length: f64,
    pub title: String,
    pub file: String,
}
