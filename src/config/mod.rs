use crate::prompt::Prompt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct History {
    pub size: u32,
    pub file_size: u32,
    pub path: String,
    pub time_format: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub history: History,
    pub prompt: Prompt,
    pub env: HashMap<String, String>,
}

impl Config {
    pub fn load(&mut self) {
        let home = match env::var("HOME") {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Cannot determine user HOME directory.");
                self.reset();
                return;
            }
        };

        let config_file_path = format!("{}/{}", home, crate::globals::CONF_FILE_NAME);

        let file = OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .open(&config_file_path);
        match file {
            Ok(x) => match serde_yaml::from_reader(x) {
                Ok(x) => *self = x,
                Err(_) => {
                    eprintln!("Config file is corrupted.");
                    self.reset();
                }
            },
            Err(_) => {
                eprintln!("No config file found.");
                self.reset();
                eprintln!("Generating new config file under {}", config_file_path);
                self.save();
            }
        }
    }

    pub fn save(&self) {
        let home = match env::var("HOME") {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Cannot determine user HOME directory.");
                return;
            }
        };

        let config_file_path = format!("{}/{}", home, crate::globals::CONF_FILE_NAME);

        let file = OpenOptions::new()
            .write(true)
            .read(false)
            .create(true)
            .open(&config_file_path);
        match file {
            Ok(x) => {
                if let Err(x) = serde_yaml::to_writer(&x, &self) {
                    eprintln!("Failed to serialize config. Reason: {}", x);
                }
            }
            Err(x) => eprintln!(
                "Failed to create config file under {}. Reason: {}",
                config_file_path, x
            ),
        }
    }

    fn reset(&mut self) {
        eprintln!("Loading default config");
        *self = Config::default();
    }
}

#[test]
fn test_serialize_deserialize() {
    use std::fs::{remove_file, OpenOptions};
    use std::io::Seek;
    use std::io::SeekFrom;

    let test_config_file_path = "config_test.yaml";

    let conf = Config::default();
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(test_config_file_path)
        .unwrap();
    serde_yaml::to_writer(&file, &conf).unwrap();
    let _ = file.seek(SeekFrom::Start(0));
    let conf: Config = serde_yaml::from_reader(&file).unwrap();
    assert_eq!(conf, Config::default());
    drop(file);
    let _ = remove_file("config_test.yaml");
}

#[test]
fn test_config() {
    use std::fs::OpenOptions;

    let test_config_file_path = "config_test.yaml";

    let conf = Config::default();
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(test_config_file_path)
        .unwrap();
    serde_yaml::to_writer(&file, &conf).unwrap();

    // let file = OpenOptions::new().write(true).read(true).create(true).open(test_config_file_path).unwrap();
    // let conf: Config = serde_yaml::from_reader(&file).unwrap();
    // println!("{:?}", conf);
}
