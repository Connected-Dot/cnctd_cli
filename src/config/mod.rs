use std::{path::Path, fs::File, io::Read, io::Write, fmt};
use cnctd::cnctd_dialogue::Dialog;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::get_exe_dir;

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, Default, PartialEq)]
enum MainOptions {
    #[default]
    UpdateGitToken,
    UpdateIphoneId,
    UpdateShortcut1,
    UpdateShortcut2,
    UpdateShortcut3,
    Exit,
}

impl fmt::Display for MainOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::UpdateGitToken => "Update Git token",
            Self::UpdateIphoneId => "Update iPhone ID",
            Self::UpdateShortcut1 => "Update shortcut 1",
            Self::UpdateShortcut2 => "Update shortcut 2",
            Self::UpdateShortcut3 => "Update shortcut 3",
            Self::Exit => "Exit",
        };
        write!(f, "{}", display_str)
    }
}


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
        let path_str = Self::get_file_path();
        let path = Path::new(&path_str);
        
        // Create the directory if it doesn't exist
        if let Some(dir) = path.parent() {
            if !dir.exists() {
                std::fs::create_dir_all(dir).expect("Failed to create directory");
            }
        }
    
        println!("path: {}", path_str.blue());
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

    pub fn get_git_token() -> Option<String> {
        match Self::get() {
            Some(config) => {
                config.git_token
            }
            None => None
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

    pub fn get_iphone_id() -> Option<String> {
        match Self::get() {
            Some(config) => {
                match config.iphone {
                    Some(iphone_id) => Some(iphone_id),
                    None => None
                }
            }
            None => None,
        }
    }

    pub fn set_iphone_id(iphone_id: &str) {
        match Self::get() {
            Some(mut config) => {
                config.iphone = Some(iphone_id.into());
                Self::write(config);
            }
            None => {
                let mut config = Self::new();
                config.iphone = Some(iphone_id.into());
                Self::write(config)
            }
        }
    }

    pub fn launch_config_setup() {
        println!("{}", "Welcome to config setup".cyan().bold());
        let prompt = "What would you like to do?";
        let selected_option = Dialog::select::<MainOptions>(prompt, None, None, None);
    
        match selected_option {
            MainOptions::UpdateGitToken => {
                update_value(Config::get_git_token(), "Git token", Config::set_git_token);
            },
            MainOptions::UpdateIphoneId => {
                update_value(Config::get_iphone_id(), "iPhone ID", Config::set_iphone_id);
            },
            MainOptions::UpdateShortcut1 => {
                update_value(Config::get_shortcut(1), "Shortcut 1", |v| Config::set_shortcut(1, v));
            },
            MainOptions::UpdateShortcut2 => {
                update_value(Config::get_shortcut(2), "Shortcut 2", |v| Config::set_shortcut(2, v));
            },
            MainOptions::UpdateShortcut3 => {
                update_value(Config::get_shortcut(3), "Shortcut 3", |v| Config::set_shortcut(3, v));
            },
            MainOptions::Exit => {},
        }
    }
}

fn update_value<T: ToString>(current_value: Option<T>, prompt: &str, update_fn: impl Fn(&str)) {
    let prompt = match &current_value {
        Some(current) => {
            println!("Current value is: {}", current.to_string().blue());
            format!("Update {}", prompt)
        }
        None => {
            println!("No current value");
            format!("Add {}", prompt)
        }
    };
    let new_value = Dialog::input::<String>(&prompt, None, None, None);
    update_fn(&new_value);
}
