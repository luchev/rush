use serde::{Serialize, Deserialize};
use crate::prompt::Prompt;
use std::collections::HashMap;

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
    // TODO
}

#[test]
fn test_serialize_deserialize() {
    use serde_yaml;
    use std::fs::{OpenOptions, remove_file};
    use std::io::SeekFrom;
    use std::io::Seek;

    let test_config_file_path = "config_test.yaml";

    let conf = Config::default();
    let mut file = OpenOptions::new().write(true).read(true).create(true).open(test_config_file_path).unwrap();
    serde_yaml::to_writer(&file, &conf).unwrap();
    let _ = file.seek(SeekFrom::Start(0));
    let conf: Config = serde_yaml::from_reader(&file).unwrap();
    assert_eq!(conf, Config::default());
    drop(file);
    let _ = remove_file("config_test.yaml");
}

#[test]
fn test_config() {
    use serde_yaml;
    use std::fs::{OpenOptions};

    let test_config_file_path = "config_test.yaml";

    let conf = Config::default();
    let file = OpenOptions::new().write(true).read(true).create(true).open(test_config_file_path).unwrap();
    serde_yaml::to_writer(&file, &conf).unwrap();

    // let file = OpenOptions::new().write(true).read(true).create(true).open(test_config_file_path).unwrap();
    // let conf: Config = serde_yaml::from_reader(&file).unwrap();
    // println!("{:?}", conf);
    
}
