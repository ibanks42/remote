use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

mod pipe;

pub async fn toggle_pause() {
    tracing::debug!("toggling pause");
    #[cfg(windows)]
    let client = pipe::get_client();

    #[cfg(unix)]
    let client = pipe::get_client().await;

    if client.is_err() {
        tracing::debug!("Error getting client: {:?}", client.err().unwrap());
        return;
    }

    let mut client = client.unwrap();

    let is_paused = pipe::get_property(&mut client, "pause")
        .await
        .as_bool()
        .unwrap();

    tracing::debug!("is_paused: {}", !is_paused);

    pipe::set_bool_property(&mut client, "pause", !is_paused).await;
}

pub async fn set_subtitle(id: i64) {
    #[cfg(windows)]
    let client = pipe::get_client();

    #[cfg(unix)]
    let client = pipe::get_client().await;

    if client.is_err() {
        tracing::debug!("Error getting client: {:?}", client.err().unwrap());
        return;
    }

    let mut client = client.unwrap();

    pipe::set_property(&mut client, "sid", id.to_string().as_str()).await;
}

pub async fn set_volume(volume: i16) {
    #[cfg(windows)]
    let client = pipe::get_client();
    #[cfg(unix)]
    let client = pipe::get_client().await;

    if client.is_err() {
        tracing::debug!("Error getting client: {:?}", client.err().unwrap());
        return;
    }

    let mut client = client.unwrap();

    pipe::set_property(&mut client, "volume", volume.to_string().as_str()).await;
}

pub async fn skip_backward() {
    #[cfg(windows)]
    let client = pipe::get_client();
    #[cfg(unix)]
    let client = pipe::get_client().await;

    if client.is_err() {
        tracing::debug!("Error getting client: {:?}", client.err().unwrap());
        return;
    }

    let mut client = client.unwrap();

    let position = pipe::get_property(&mut client, "time-pos").await.as_f64();
    if position.is_none() {
        tracing::debug!("Error getting position");
        return;
    }
    let position = position.unwrap().sub(10.0f64);

    pipe::set_property(&mut client, "time-pos", position.to_string().as_str()).await;
}

pub async fn skip_forward() {
    #[cfg(windows)]
    let client = pipe::get_client();
    #[cfg(unix)]
    let client = pipe::get_client().await;

    if client.is_err() {
        tracing::debug!("Error getting client: {:?}", client.err().unwrap());
        return;
    }

    let mut client = client.unwrap();

    let position = pipe::get_property(&mut client, "time-pos").await.as_f64();
    if position.is_none() {
        tracing::debug!("Error getting position");
        return;
    }
    let position = position.unwrap().add(10.0f64);

    pipe::set_property(&mut client, "time-pos", position.to_string().as_str()).await;
}

pub async fn volume_up() {
    #[cfg(windows)]
    let client = pipe::get_client();
    #[cfg(unix)]
    let client = pipe::get_client().await;

    if client.is_err() {
        tracing::debug!("Error getting client: {:?}", client.err().unwrap());
        return;
    }

    let mut client = client.unwrap();

    let volume = pipe::get_property(&mut client, "volume").await.as_f64();
    if volume.is_none() {
        tracing::debug!("Error getting volume");
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
        tracing::debug!("Error getting client: {:?}", client.err().unwrap());
        return;
    }
    let mut client = client.unwrap();

    let volume = pipe::get_property(&mut client, "volume").await.as_f64();
    if volume.is_none() {
        tracing::debug!("Error getting volume");
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

    let subtitle = pipe::get_property(&mut client, "sid").await.to_string();

    let track_list_count = pipe::get_property(&mut client, "track-list/count")
        .await
        .as_i64()
        .unwrap();

    let mut subtitles: Vec<Subtitle> = vec![];
    for i in 0..track_list_count {
        let track_type = pipe::get_property(&mut client, format!("track-list/{}/type", i).as_str())
            .await
            .to_string();
        if track_type == "\"sub\"" {
            let id = pipe::get_property(&mut client, format!("track-list/{}/id", i).as_str())
                .await
                .as_i64()
                .unwrap();
            let lang = pipe::get_property(&mut client, format!("track-list/{}/lang", i).as_str())
                .await
                .to_string();
            let title = pipe::get_property(&mut client, format!("track-list/{}/title", i).as_str())
                .await
                .to_string();

            let mut result = lang.clone();

            if title != "null" {
                result = format!("{} - {}", lang, title);
            }
            result = result.replace("\"", "");

            subtitles.push(Subtitle { id, title: result })
        }
    }

    Ok(Status {
        paused,
        volume,
        position,
        length,
        title,
        file,
        subtitle,
        subtitles,
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subtitle {
    pub id: i64,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub paused: bool,
    pub volume: f64,
    pub position: f64,
    pub length: f64,
    pub title: String,
    pub file: String,
    pub subtitle: String,
    pub subtitles: Vec<Subtitle>,
}
