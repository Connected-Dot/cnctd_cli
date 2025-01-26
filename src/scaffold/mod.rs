use std::fmt;
use cnctd::{cnctd_dialogue::Dialog, cnctd_git::account::GitAccount, cnctd_cargo::cargo_toml::Author, cnctd_utils::display_logo};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::{config::{git_config::GitConfig, Config}, scaffold::module::ModuleScaffold, scripts::Scripts};

use self::project::ProjectScaffold;

pub mod apps;
pub mod project;
pub mod module;

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, Default, PartialEq)]
pub enum MainOptions {
    #[default]
    LaunchNewProject,
    CreateModule,
    Config,
    RunScript,
    Exit,
}

impl fmt::Display for MainOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::LaunchNewProject => "Launch new project",
            Self::CreateModule => "Create module",
            Self::Config => "Config",
            Self::RunScript => "Run Script",
            Self::Exit => "Exit",
        };
        write!(f, "{}", display_str)
    }
}

pub struct Scaffold {}

impl Scaffold {
    pub async fn run() -> anyhow::Result<()> {
        display_logo("cnctd", true);
        loop {

            let main_selection = Dialog::select::<MainOptions>("What would you like to do?", None, None, None);

            match main_selection {
                MainOptions::LaunchNewProject=> {
                    let mut project = ProjectScaffold::new();
                    loop {
                        println!("\n");
                        // Get the project name
                        project
                            .get_app_settings()
                            .set_name()
                            .set_project_dir()
                            .build().await?;

                        break;  // This will exit the inner loop and go back to the main menu
                    }
                },
                MainOptions::CreateModule => {
                    let mut module = ModuleScaffold::new();
                    loop {
                        println!("\n");

                        module 
                            .set_name()
                            .set_module_directory()
                            .set_description()
                            .set_is_private()
                            .build().await?;

                        break;
                    }
                }
                MainOptions::Config => {
                    Config::launch_config_setup().await?;
                },
                MainOptions::RunScript => {
                    Scripts::launch_scripts_menu().await?;
                },
                MainOptions::Exit => {
                    break;
                },
            }
        }
        Ok(())
    }

    pub async fn select_git_account() -> anyhow::Result<GitAccount> {
        let mut config = Config::get()?;
        let default_account = match config.git.get_default_account() {
            Some(account) => {
                account
            },
            None => {
                println!("\n{}\n{}\n", "No Git accounts configured".yellow(), "configure now".yellow());
                let new_account = GitConfig::add_account(&mut config.git).await?;
                config.write()?;
                new_account
            }
        };

        let accounts = &config.git.get_accounts();
        let mut account_urls: Vec<&str> = vec![];
        
        for acc in accounts {
            account_urls.push(&acc.personal_url);
            for org_url in &acc.org_urls {
                account_urls.push(&org_url);
            }
        }
        let prompt = "Choose the Git URL";
        let default_index = account_urls.iter().position(|&url| url == &default_account.default_url).unwrap();
        let selected_url = Dialog::select_str(prompt, &account_urls, Some(default_index), None, None);
        
        println!("selected URL: {}", selected_url);
        let selected_account = accounts.iter().find(|&acc| {
            acc.personal_url == selected_url || acc.org_urls.contains(&selected_url.to_string())
        }).unwrap();

        Ok(selected_account.clone())
    }

    pub async fn select_authors() -> anyhow::Result<Author> {
        let mut config = Config::get()?;
        let mut new_authors: Vec<Author> = vec![];
        let authors = match &config.cargo_toml.authors {
            Some(authors) => authors,
            None => {
                println!("\n{}\n{}\n", "No authors configured".yellow(), "configure now".yellow());
                let new_author = config.cargo_toml.add_author()?;
                config.write()?;
                new_authors.push(new_author);
                &new_authors
            }
        };
        let mut author_emails: Vec<&str> = vec![];
        
        for auth in authors {
           author_emails.push(&auth.email)
        }

        let prompt = "Choose the author";
        let default_email = &config.cargo_toml.default_author.unwrap();
        let default_index = author_emails.iter().position(|&email| email.to_string() == default_email.to_string()).unwrap();
        let selected_email = Dialog::select_str(prompt, &author_emails, Some(default_index), None, None);

        let selected_author = authors.iter().find(|&auth| {
            auth.email == selected_email
        }).unwrap();

        Ok(selected_author.clone())
    }   

    pub fn select_license() -> anyhow::Result<String> {
        let mut config = Config::get()?;
        let license;
        let license = match &config.cargo_toml.default_license {
            Some(license) => license,
            None => {
                license = config.cargo_toml.set_default_license()?;
                &license
            }
        };

        Ok(license.to_string())
    }
}
