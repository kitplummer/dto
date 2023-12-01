use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub app_id: String,
    pub license_token: String,
    pub repaint: bool,
}

pub fn show_configuration_file() {
    let path = find_default_config_file().unwrap();
    let config = read_config(&path);
    println!("{}", serde_json::json!(&config));
}

pub fn read_config(path: &Path) -> Config {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = toml::from_str(&contents).unwrap();
    config
}

pub fn create_configuration_file() {
    write_default_config().expect("Could not write default config file");
    println!("Wrote the default configuration file at ~/.config/dto.toml");
}

pub fn write_default_config() -> Result<(), ()> {
    let config = Config {
        app_id: "".to_string(),
        license_token: "".to_string(),
        repaint: false,
    };

    let toml = toml::to_string(&config).unwrap();

    let config_path = find_default_config_file().unwrap();
    let prefix = config_path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let config_path = config_path.into_os_string().into_string().unwrap();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config_path)
        .unwrap();

    write!(&mut file, "{}", toml).expect("Could not write default configuration file");
    Ok(())
}

pub fn config_file_exists() -> bool {
    let config_file_path = find_default_config_file().unwrap();
    config_file_path.as_path().exists()
}

pub fn find_default_config_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".config");
        path.push("dto.toml");
        path
    })
}
