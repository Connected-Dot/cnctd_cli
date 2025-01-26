use std::{path::Path, fs::{File, remove_file}, io::Read, io::Write, fmt};
use async_recursion::async_recursion;
use cnctd_dialogue::Dialog;
use cnctd_utils::{display_logo, get_exe_dir};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json;
use anyhow::anyhow;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use self::{git_config::GitConfig, cargo_toml_config::CargoTomlConfig, shortcut::Shortcut, device_config::{DeviceType, DeviceConfig}};

pub mod git_config;
pub mod cargo_toml_config;
pub mod shortcut;
pub mod device_config;

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, Default, PartialEq)]
enum MainOptions {
    #[default]
    Git,
    CargoToml,
    Devices,
    Shortcuts,
    ClearConfig,
    Blank,
    Back,
    Exit,
}

impl fmt::Display for MainOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::Git => "Git accounts",
            Self::CargoToml => "Cargo.toml",
            Self::Devices => "Devices",
            Self::Shortcuts => "Shortcuts",
            Self::ClearConfig => "Clear config",
            Self::Blank => "- - - -",
            Self::Back => "Back",
            Self::Exit => "Exit",
        };
        write!(f, "{}", display_str)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub git: GitConfig,
    pub cargo_toml: CargoTomlConfig,
    pub devices: DeviceConfig,
    pub shortcuts: Vec<Shortcut>
}

impl Config {
    pub fn new() -> Self {
        Self {
            git: GitConfig { git_accounts: None, default_account: None },
            cargo_toml: CargoTomlConfig { authors: None, default_author: None, default_license: None },
            devices: DeviceConfig { ios: vec![], android: vec![], default_android: None, default_ios: None },
            shortcuts: vec![],
        }
    }

    pub fn get_file_path() -> String {
        let exe_dir = get_exe_dir();
        // println!("executable directory is {:?}", exe_dir);
        format!("{}/cnctd_config.json", exe_dir).replace("/cnctd/", "/")
    }

    pub fn get() ->  anyhow::Result<Self> {
        let config_path = Self::get_file_path();
        let path = Path::new(&config_path);
        let mut config_file = File::open(path)?;
        let mut contents = String::new();

        config_file.read_to_string(&mut contents)?;

        match serde_json::from_str(&contents.as_str()) {
            Ok(config) => Ok(config),
            Err(e) => {
                println!("error: {}", e.to_string().red());
                let config = Self::new();
                config.write()?;
                Ok(config)
            }
        }
    }

    pub fn write(&self) -> anyhow::Result<File> {
        let config_string = serde_json::to_string(&self).unwrap();
        let path_str = Self::get_file_path();
        let path = Path::new(&path_str);
        
        // Create the directory if it doesn't exist
        if let Some(dir) = path.parent() {
            if !dir.exists() {
                std::fs::create_dir_all(dir).expect("Failed to create directory");
            }
        }
        // println!("path: {}", path_str.blue());
        match File::create(&path) {
            Ok(mut file) => {
                let buf = config_string.as_bytes();
                file.write(buf).unwrap();
                Ok(file)
            }
            Err(e) => {
                let error = format!("error: {}", e);
                println!("{}", error.red());
                Err(anyhow!(error))
            }
        }
    }

    pub fn clear(&self) {
        let path_str = Self::get_file_path();
        let path = Path::new(&path_str);

        if path.exists() {
            match remove_file(&path) {
                Ok(_) => {
                    println!("Successfully deleted config file at {}", path_str.blue());
                }
                Err(e) => {
                    let error = format!("Failed to delete config file: {}", e);
                    println!("{}", error.red());
                }
            }
        } else {
            println!("No config file exists at {}", path_str.blue());
        }
    }

    #[async_recursion]
    pub async fn launch_config_setup() -> anyhow::Result<()> {
        display_logo("config", false);
        let mut config = Self::get()?;
        let prompt = "Which settings would you like to update?";
        let selected_option = Dialog::select::<MainOptions>(prompt, None, None, None);
    
        match selected_option {
            MainOptions::Git => config.manage_git_accounts().await?,
            MainOptions::CargoToml => config.manage_cargo_toml().await?,
            MainOptions::Devices => config.manage_devices().await?,
            MainOptions::Shortcuts => config.manage_shortcuts().await?,
            MainOptions::ClearConfig => config.clear(),
            MainOptions::Blank => Self::launch_config_setup().await?,
            MainOptions::Back => {},
            MainOptions::Exit => std::process::exit(0),
            
        }
        Ok(())
    }

    #[async_recursion]
    pub async fn manage_git_accounts(&mut self) -> anyhow::Result<()> {
        display_logo("GitHub", false);
        self.git.display_accounts();
        let prompt = "What would you like to do?";
        let options = vec!["Add", "Remove", "Set default account", "Set default URL", "Back", "Exit"];
        let selection = Dialog::select_str(prompt, &options, None, None, None);
        match &*selection {
            "Add" => {
                self.git.add_account().await?;
                self.write()?;
                self.manage_git_accounts().await?;
            },
            "Remove" => {
                self.git.remove_account()?;
                self.write()?;
                self.manage_git_accounts().await?;
            }
            "Set default account" => {
                self.git.set_default_account()?;
                self.write()?;
                self.manage_git_accounts().await?;
            }
            "Set default URL" => {
                self.git.set_default_url()?;
                self.write()?;
                self.manage_git_accounts().await?;
            },
            "Back" => Self::launch_config_setup().await?,
            &_ => std::process::exit(0)
        }
        Ok(())
    }

    
    
    #[async_recursion]
    pub async fn manage_cargo_toml(&mut self) -> anyhow::Result<()> {
        display_logo("Cargo.toml", false);
        self.cargo_toml.display();
        let prompt = "What would you like to do?";
        let options = vec!["Add author", "Remove author", "Set Default Author", "Set Default License", "Back", "Exit"];
        let selection = Dialog::select_str(prompt, &options, None, None, None);
        match &*selection {
            "Add author" => {
                self.cargo_toml.add_author()?;
                self.write()?;
                self.manage_cargo_toml().await?;
            },
            "Remove author" => {
                self.cargo_toml.remove_author();
                self.write()?;
                self.manage_cargo_toml().await?;
            }
            "Set Default Author" => {
                self.cargo_toml.set_default_author();
                self.write()?;
                self.manage_cargo_toml().await?;
            }
            "Set Default License" => {
                self.cargo_toml.set_default_license()?;
                self.write()?;
                self.manage_cargo_toml().await?;
            },
            "Back" => Self::launch_config_setup().await?,
            &_ => std::process::exit(0)
        }

        Ok(())
    }


    #[async_recursion]
    pub async fn manage_devices(&mut self) -> anyhow::Result<()> {
        display_logo("devices", false);
        self.devices.display_devices();
        let prompt = "What would you like to do?";
        let options = vec!["Add device", "Remove device", "Set default iOS device", "Set default Android device", "Back", "Exit"];
        let selection = Dialog::select_str(prompt, &options, None, None, None);
        match &*selection {
            "Add device" => {
                self.devices.add_device();
                self.write()?;
                self.manage_devices().await?;
            },
            "Remove device" => {
                self.devices.remove_device();
                self.write()?;
                self.manage_devices().await?;
            }
            "Set default iOS device" => {
                self.devices.set_default_device(DeviceType::Ios);
                self.write()?;
                self.manage_devices().await?;
            }
            "Set default Android Device" => {
                self.devices.set_default_device(DeviceType::Android);
                self.write()?;
                self.manage_devices().await?;
            },
            "Back" => Self::launch_config_setup().await?,
            &_ => std::process::exit(0)
        }

        Ok(())
    } 

    

    #[async_recursion]
    pub async fn manage_shortcuts(&mut self) -> anyhow::Result<()> {
        display_logo("shortcuts", false);
        Shortcut::display_all(&mut self.shortcuts);
        let prompt = "What would you like to do?";
        let options = vec!["Add shortcut", "Remove shortcut", "Execute shortcut", "Back", "Exit"];
        let selection = Dialog::select_str(prompt, &options, None, None, None);
        match &*selection {
            "Add shortcut" => {
                Shortcut::add(&mut self.shortcuts);
                self.write()?;
                self.manage_shortcuts().await?;
            },
            "Remove shortcut" => {
                Shortcut::remove(&mut self.shortcuts);
                self.write()?;
                self.manage_shortcuts().await?;
            }
            "Execute shortcut" => {
                let mut shortcut_names: Vec<&str> = vec![];
                for shortcut in &self.shortcuts { shortcut_names.push(&shortcut.name) }
                let name = Dialog::select_str("Choose Shortcut", &shortcut_names, None, None, None);
                Shortcut::execute(&name).await?;
            }
            "Back" => Self::launch_config_setup().await?,
            &_ => std::process::exit(0)
        }

        Ok(())
    } 

}


