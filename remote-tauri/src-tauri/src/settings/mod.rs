use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Settings {
    pub mpv: MpvSettings,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct MpvSettings {
    pub pipe: String,
}

#[tauri::command]
pub fn save_settings(settings: &str) {
    println!("Saving settings {}", settings);
    let settings = serde_json::from_str::<Settings>(settings);
    if settings.is_err() {
        panic!("Error parsing settings");
    }
    let settings = settings.unwrap();

    write_settings(&settings);
}

#[tauri::command]
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
        println!("Error parsing settings");
        let settings = get_default_settings();
        write_settings(&settings);
        return settings;
    }

    deserialized.unwrap()
}

fn write_settings(settings: &Settings) {
    println!("Writing settings");
    let mut writer = std::fs::File::create("settings.json").unwrap();

    println!("Writing settings");
    let pretty = serde_json::to_string_pretty(&settings);
    if pretty.is_err() {
        panic!("Error writing settings")
    }

    let write = writer.write(pretty.unwrap().as_bytes());

    if write.is_err() {
        panic!("Error writing settings")
    }
}

fn get_default_settings() -> Settings {
    Settings {
        mpv: MpvSettings {
            pipe: r"\\.\pipe\mpvpipe".to_string(),
        },
        port: 7920,
    }
}
