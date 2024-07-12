use std::ops::{Add, Sub};

mod pipe;

pub async fn toggle_pause() {
    let client = pipe::get_client();

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
    let client = pipe::get_client();

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
    let client = pipe::get_client();

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
