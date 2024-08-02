use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Settings {
    pub port: u16,
    pub mpv: Option<MpvSettings>,
    pub autohide: Option<bool>,
    pub window_size: Option<(u32, u32)>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct MpvSettings {
    pub pipe: String,
}

// Create a new settings file with default values if it doesn't exist.
// Location:
// UNIX: ~/.config/home-remote/settings.json
// WINDOWS: %APPDATA%/home-remote/settings.json

#[cfg(windows)]
pub fn get_settings_path() -> String {
    let appdata = std::env::var("APPDATA").unwrap();
    format!("{}/home-remote/settings.json", appdata)
}

#[cfg(unix)]
pub fn get_settings_path() -> String {
    let home = std::env::var("HOME").unwrap();
    format!("{}/.config/home-remote/settings.json", home)
}

#[tauri::command]
pub fn save_settings(settings: &str) {
    tracing::debug!("Saving settings {}", settings);
    let settings = serde_json::from_str::<Settings>(settings);
    if settings.is_err() {
        panic!("Error parsing settings: {}", settings.err().unwrap());
    }
    let settings = settings.unwrap();

    write_settings(&settings);

    tracing::debug!("Settings saved");
}

#[tauri::command]
pub fn load_settings() -> Settings {
    let settings_file = std::fs::File::open(get_settings_path());

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
        tracing::debug!("Error parsing settings: {}", deserialized.err().unwrap());
        let settings = get_default_settings();
        write_settings(&settings);
        return settings;
    }

    deserialized.unwrap()
}

pub fn write_settings(settings: &Settings) {
    // create directory if it doesn't exist
    let path = get_settings_path();
    let dir = std::path::Path::new(&path).parent().unwrap();
    if !dir.exists() {
        std::fs::create_dir_all(dir).unwrap();
    }

    let mut writer = std::fs::File::create(path).unwrap();

    tracing::debug!("Writing settings");
    let pretty = serde_json::to_string_pretty(&settings);
    if pretty.is_err() {
        panic!("Error writing settings: {}", pretty.err().unwrap());
    }

    let write = writer.write(pretty.unwrap().as_bytes());

    if write.is_err() {
        panic!("Error writing settings: {}", write.err().unwrap());
    }

    tracing::debug!("Settings written: {} bytes", write.unwrap());
}

fn get_default_settings() -> Settings {
    Settings {
        port: 7920,
        #[cfg(windows)]
        mpv: MpvSettings {
            pipe: r"\\.\pipe\mpvpipe".to_string(),
        },
        #[cfg(unix)]
        mpv: Some(MpvSettings {
            pipe: r"/tmp/mpvsocket".to_string(),
        }),
        autohide: Some(true),
        window_size: Some((320, 600)),
    }
}
