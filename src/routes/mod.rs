use std::env::current_dir;

use cnctd::cnctd_bump::bump_project;
use colored::Colorize;
// use cnctd_git::Git;

use crate::{Commands, scaffold::Scaffold, display_logo, project::print_project_versions, config::{Config, shortcut::Shortcut}};


// use self::commands::config::{route_config_command, ConfigOptions};

// pub mod commands;

pub async fn route_command(command: Option<Commands>) -> anyhow::Result<()> {
    match command {
        Some(Commands::Config {} ) => {
            Config::launch_config_setup().await?;
        }
        Some(Commands::New {  }) => {
            Scaffold::run().await?;
        }
        Some(Commands::Update {  }) => {
            // let git_token = Config::get_git_token().unwrap();
            // let git = Git::new(git_token);
            // git.list_all_repos().await.unwrap();
            // git.test_git2_auth("https://github.com/Connected-Dot/cnctd_git").unwrap();
        }
        Some(Commands::S { name }) => {
            Shortcut::execute(&name).await?;
        }
        Some(Commands::Bump { version_part }) => {
            match version_part {
                Some(version_part) => bump_project(&version_part).await?,
                None => bump_project("patch").await?,
            }
            
        }
        Some(Commands::Versions { d }) => {
            match d {
                Some(d) => {
                    print_project_versions(&d)?;
                }
                None => {
                    let current_path = current_dir()?;
                    println!("current path: {:?}", current_path);
                    let dir = current_path.as_os_str().to_str().unwrap();
                    print_project_versions(dir)?;
                }
            }
        }
        None => {
            Scaffold::run().await?;
        }
    }
    Ok(())
}