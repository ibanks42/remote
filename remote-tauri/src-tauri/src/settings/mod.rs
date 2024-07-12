use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Settings {
    pub mpv: MpvSettings,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct MpvSettings {
    pub pipe: String,
}

#[tauri::command]
pub fn save_settings(settings: &str) {
    let settings = serde_json::from_str::<Settings>(settings);
    if settings.is_err() {
        panic!("Error parsing settings");
    }
    let settings = settings.unwrap();

    let writer = std::fs::File::create("settings.json").unwrap();

    let write = serde_json::to_writer(writer, &settings);

    if write.is_err() {
        panic!("Error writing settings")
    }

    write.unwrap()
}

pub fn load_settings() -> Settings {
    let settings_file = std::fs::File::open("settings.json");

    if settings_file.is_err() {
        return get_default_settings();
    }

    let mut buf: Vec<u8> = Vec::new();
    let reader = settings_file.unwrap().read_to_end(&mut buf);

    if reader.is_err() {
        return get_default_settings();
    }

    let deserialized = serde_json::from_slice::<Settings>(&buf);

    if deserialized.is_err() {
        return get_default_settings();
    }

    deserialized.unwrap()
}

fn get_default_settings() -> Settings {
    Settings {
        mpv: MpvSettings {
            pipe: r"\\.\pipe\mpvpipe".to_string(),
        },
    }
}
