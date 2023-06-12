use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{Deserialize, Serialize};

pub fn load_config() -> Config {
    let path = Path::new("config.json");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(error) => panic!("Could not load config from {}: {}", display, error),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(error) => panic!("Could not read config from {}: {}", display, error),
        Ok(_) => { }
    }

    match serde_json::from_str(&file_contents) {
        Err(error) => panic!("Invalid json in {}: {}", display, error),
        Ok(settings) => settings,
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub file_path: String,
    pub player_names: Vec<String>,
}