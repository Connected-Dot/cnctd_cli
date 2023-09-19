use std::{path::Path, fs::File, io::Read, io::Write};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::get_exe_dir;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub git_token: Option<String>,
    pub iphone: Option<String>,
    pub shortcut_1: Option<String>,
    pub shortcut_2: Option<String>,
    pub shortcut_3: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            git_token: None,
            iphone: None,
            shortcut_1: None,
            shortcut_2: None,
            shortcut_3: None,
        }
    }

    pub fn get_file_path() -> String {
        let exe_dir = get_exe_dir();
        // println!("executable directory is {:?}", exe_dir);
        format!("{}/cnctd_config.json", exe_dir).replace("/cnctd/", "/")
    }

    pub fn get() ->  Option<Self> {
        let config_path = Self::get_file_path();
        let path = Path::new(&config_path);
        let mut config_file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return None,
        };
        let mut contents = String::new();

        config_file.read_to_string(&mut contents).unwrap();
        match serde_json::from_str(&contents) {
            Ok(config) => Some(config),
            Err(e) => {
                println!("error: {}", e.to_string().red());
                None
            }
        }
    }

    pub fn write(self) {
        let config_string = serde_json::to_string(&self).unwrap();
        let path = Self::get_file_path();
        println!("path: {}", path.blue());
        match File::create(&path) {
            Ok(mut file) => {
                let buf = config_string.as_bytes();
                file.write(buf).unwrap();
            }
            Err(e) => {
                let error = format!("error: {}", e);
                println!("{}", error.red());
            }
        }
    }

    pub fn set_git_token(token: &str) {
        match Self::get() {
            Some(mut config) => {
                config.git_token = Some(token.into());
                Self::write(config);
            }
            None => {
                let mut config = Self::new();
                config.git_token = Some(token.into());
                Self::write(config)
            }
        }
    }

    pub fn get_shortcut(id: u8) -> Option<String> {
        match Self::get() {
            Some(config) => {
                match id {
                    1 => {
                        match config.shortcut_1 {
                            Some(shortcut) => Some(shortcut),
                            None => None
                        }
                    }
                    2 => {
                        match config.shortcut_2 {
                            Some(shortcut) => Some(shortcut),
                            None => None
                        }
                    }
                    3 => {
                        match config.shortcut_3 {
                            Some(shortcut) => Some(shortcut),
                            None => None
                        }
                    }
                    _ => None
                }
            }
            None => None
        }
    }

    pub fn set_shortcut(id: u8, shortcut: &str) {
        match Self::get() {
            Some(mut config) => {
                match id {
                    1 => {
                        config.shortcut_1 = Some(shortcut.into());
                        Self::write(config);
                    }
                    2 => {
                        config.shortcut_2 = Some(shortcut.into());
                        Self::write(config);
                    }
                    3 => {
                        config.shortcut_3 = Some(shortcut.into());
                        Self::write(config);
                    }
                    _ => {}
                }
            }
            None => {
                let mut config = Self::new();
                match id {
                    1 => {
                        config.shortcut_1 = Some(shortcut.into());
                        Self::write(config);
                    }
                    2 => {
                        config.shortcut_2 = Some(shortcut.into());
                        Self::write(config);
                    }
                    3 => {
                        config.shortcut_3 = Some(shortcut.into());
                        Self::write(config);
                    }
                    _ => {}
                }
            }
        }
    }
}