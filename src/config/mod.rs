use std::{path::Path, fs::{File, remove_file}, io::Read, io::Write, fmt, env::current_dir};
use async_recursion::async_recursion;
use cnctd::{cnctd_dialogue::Dialog, cnctd_git::{account::GitAccount, GitProvider}};
use colored::{Colorize, ColoredString};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{get_exe_dir, print_separator, project::update_rust_project_versions};

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, Default, PartialEq)]
enum MainOptions {
    #[default]
    Git,
    Devices,
    Shortcuts,
    ClearConfig,
    UpdateDependencies,
    Exit,
}

impl fmt::Display for MainOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::Git => "Manage Git accounts",
            Self::Devices => "Manage devices",
            Self::Shortcuts => "Manage shortcuts",
            Self::ClearConfig => "Clear config",
            Self::UpdateDependencies => "Update dependencies",
            Self::Exit => "Exit",
        };
        write!(f, "{}", display_str)
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub git_accounts: Option<Vec<GitAccount>>,
    pub default_git_account: Option<String>,
    pub iphone: Option<String>,
    pub shortcut_1: Option<String>,
    pub shortcut_2: Option<String>,
    pub shortcut_3: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            git_accounts: None,
            default_git_account: None,
            iphone: None,
            shortcut_1: None,
            shortcut_2: None,
            shortcut_3: None,
        }
    }

    pub async fn launch_config_setup() -> anyhow::Result<()> {
        let mut config = match Self::get() {
            Some(config) => config,
            None => Self::new()
        };
        // println!("{}", "Welcome to config setup".cyan().bold());
        let prompt = "What would you like to do?";
        let selected_option = Dialog::select::<MainOptions>(prompt, None, None, None);
    
        match selected_option {
            MainOptions::Git => config.manage_git_accounts().await?,
            MainOptions::Devices => {
            },
            MainOptions::Shortcuts => {
            },
            MainOptions::ClearConfig => config.clear(),
            MainOptions::UpdateDependencies => {
                let current_path = current_dir()?;
                let dir = current_path.as_os_str().to_str().unwrap();
                update_rust_project_versions(dir)?;
            }
            MainOptions::Exit => {},
            
        }
        Ok(())
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

    pub fn write(&self) {
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
            }
            Err(e) => {
                let error = format!("error: {}", e);
                println!("{}", error.red());
            }
        }
        print_separator(50, false);
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
    pub async fn manage_git_accounts(&mut self) -> anyhow::Result<()> {
        match &self.git_accounts {
            Some(accounts) => {
                self.display_git_accounts(accounts);
                let prompt = "What would you like to do?";
                let options = ["Add", "Remove", "Set Default Account", "Set Default URL", "Back", "Exit"];
                let selection = Dialog::select_str(prompt, &options, None, None, None);
                match &*selection {
                    "Add" => {
                        self.add_git_account().await?;
                        self.manage_git_accounts().await?
                    },
                    "Remove" => {
                        self.remove_git_account()?;
                        self.manage_git_accounts().await?
                    }
                    "Set Default Account" => {
                        self.set_default_git_account()?;
                        self.manage_git_accounts().await?
                    }
                    "Set Default URL" => {
                        self.set_default_git_url()?;
                        self.manage_git_accounts().await?
                    },
                    "Back" => Self::launch_config_setup().await?,
                    &_ => {

                    }
                }
                Ok(())
            },
            None => {
                println!("No accounts configured");
                self.add_git_account().await?;
                Ok(())
            },
        }
    }

    #[async_recursion]
    pub async fn add_git_account(&mut self) -> anyhow::Result<()> {
        let prompt = "Enter your GitHub token\n";
        let token: String = Dialog::input(prompt, None, None, None);
        let git_account = GitAccount::new(GitProvider::GitHub, &token).await?;
        let login = git_account.login.clone();
    
        match &mut self.git_accounts {
            Some(accounts) => {
                if accounts.iter().any(|account| account.login == login) {
                    println!("{}", "This account already exists.".yellow());
                } else {
                    accounts.push(git_account.clone());
                    if accounts.len() == 1 {
                        self.default_git_account = Some(login);
                    }
                    self.write();
                }
            },
            None => {
                self.git_accounts = Some(vec![git_account]);
                self.default_git_account = Some(login);
                self.write();
            }
        }
        Ok(())
    }

    pub fn remove_git_account(&mut self) -> anyhow::Result<()> {
        match &mut self.git_accounts {
            Some(git_accounts) => {
                if git_accounts.is_empty() {
                    println!("No Git accounts configured");
                } else {
                    let prompt = "Which account would you like to remove?";
                    let account_names: Vec<&str> = git_accounts.iter().map(|acc| acc.login.as_str()).collect();
                    let selected_login = Dialog::select_str(prompt, &account_names, None, None, None);
    
                    if let Some(index) = git_accounts.iter().position(|acc| acc.login == selected_login) {
                        git_accounts.remove(index);
                        self.write();
                        println!("Successfully removed the selected Git account.");
                        
                    } else {
                        println!("No account selected for removal.");
                    }
                }

                
            }
            None => {
                println!("No Git accounts configured");
            }
        }
        Ok(())
    }

    fn set_default_git_account(&mut self) -> anyhow::Result<()> {
        match &mut self.git_accounts {
            Some(git_accounts) => {
                if git_accounts.is_empty() {
                    println!("No Git accounts configured");
                    return Ok(());
                }

                let prompt = "Which account would you like to set as default?";
                let account_names: Vec<&str> = git_accounts.iter().map(|acc| acc.login.as_str()).collect();
                let selected_login = Dialog::select_str(prompt, &account_names, None, None, None);

                self.default_git_account = Some(selected_login);
            }
            None => {
                println!("No Git accounts configured");
            }
        }
        Ok(())
    }

    fn set_default_git_url(&mut self) -> anyhow::Result<()> {
        match &mut self.git_accounts {
            Some(git_accounts) => {
                if git_accounts.is_empty() {
                    println!("No Git accounts configured");
                    return Ok(());
                }

                let prompt = "Which account would you like to edit default URL for?";
                let account_names: Vec<&str> = git_accounts.iter().map(|acc| acc.login.as_str()).collect();
                let selected_login = Dialog::select_str(prompt, &account_names, None, None, None);
    
                if let Some(selected_account) = git_accounts.iter_mut().find(|acc| acc.login == selected_login) {
                    let mut all_urls: Vec<&str> = vec![];
                    all_urls.push(&selected_account.default_url);
                    for org_url in &selected_account.org_urls {
                        all_urls.push(&org_url);
                    }
                    let prompt = "Choose the default URL";
                    let selected_default = Dialog::select_str(prompt, &all_urls, None, None, None);
                    selected_account.default_url = selected_default;
                    self.write();
                } else {
                    println!("Account not found.");
                }
            }
            None => {
                println!("No Git accounts configured");
            }
        }
        Ok(())
    }

    fn display_git_accounts(&self, accounts: &Vec<GitAccount>) {
        println!("\nCurrent accounts:");
        for (i, account) in accounts.iter().enumerate() {
            let default_account = match self.default_git_account.clone() {
                Some(login) => login,
                None => "".to_string()
            };
            let is_default_account = if default_account == account.login {
                " <----- DEFAULT".purple()
            } else { "".white() };
            println!("\n{}{}", format!("Account {}", i + 1).blue().bold(), is_default_account);
            println!("Login: {}", account.login);
            println!("Token: {}", account.token);
            let default_url = &account.default_url;
            fn is_default(url: &str, default_url: &str) -> ColoredString {
                if url == default_url {
                    " <----- DEFAULT".purple()
                } else { "".white() }
            }
            println!("Personal URL: {}{}", account.personal_url, is_default(&account.personal_url, default_url));
            if account.org_urls.len() > 0 { 
                for url in &account.org_urls {
                    println!("Org URL: {}{}", url, is_default(url, &default_url));
                }
            }
        }
        println!("\n");
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
                        config.write()
                    }
                    2 => {
                        config.shortcut_2 = Some(shortcut.into());
                        config.write()
                    }
                    3 => {
                        config.shortcut_3 = Some(shortcut.into());
                        config.write()
                    }
                    _ => {}
                }
            }
            None => {
                let mut config = Self::new();
                match id {
                    1 => {
                        config.shortcut_1 = Some(shortcut.into());
                        config.write()
                    }
                    2 => {
                        config.shortcut_2 = Some(shortcut.into());
                        config.write()
                    }
                    3 => {
                        config.shortcut_3 = Some(shortcut.into());
                        config.write()
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
                config.write()
            }
            None => {
                let mut config = Self::new();
                config.iphone = Some(iphone_id.into());
                config.write()
            }
        }
    }



}

// fn update_value<T: ToString>(current_value: Option<T>, prompt: &str, update_fn: impl Fn(&str)) {
//     let prompt = match &current_value {
//         Some(current) => {
//             println!("Current value is: {}", current.to_string().blue());
//             format!("Update {}", prompt)
//         }
//         None => {
//             println!("No current value");
//             format!("Add {}", prompt)
//         }
//     };
//     let new_value = Dialog::input::<String>(&prompt, None, None, None);
//     update_fn(&new_value);
// }
