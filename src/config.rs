use crate::Paths;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub targets_list: Vec<String>,
    pub min_length: usize,
}

impl Config {
    pub fn new(paths: &Paths) -> Self {
        let mut raw = String::new();

        match std::fs::File::open(&paths.config_path) {
            Ok(mut f) => {
                f.read_to_string(&mut raw).unwrap();
                toml::from_str(&raw).unwrap()
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    let r = Config::default();
                    let mut f = std::fs::File::create(&paths.config_path).unwrap();
                    f.write_all(toml::to_string_pretty(&r).unwrap().as_bytes())
                        .unwrap();

                    r
                }
                _ => std::panic::panic_any(e),
            },
        }
    }

    fn default() -> Self {
        Config {
            targets_list: ["image/png".to_owned(), "UTF8_STRING".to_owned()].to_vec(),
            min_length: 3,
        }
    }
}
