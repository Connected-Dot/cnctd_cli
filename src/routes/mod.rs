use std::env::current_dir;
use cnctd::{cnctd_bump::bump_project, cnctd_git::{repo::GitRepo, account::GitAccount, GitProvider}, cnctd_dialogue::Dialog};

use crate::{Commands, scaffold::Scaffold, project::print_project_versions, config::{Config, shortcut::Shortcut}, manager::Manager};

// pub mod commands;

pub async fn route_command(command: Option<Commands>) -> anyhow::Result<()> {
    match command {
        Some(Commands::Config {} ) => {
            Config::launch_config_setup().await?;
        }
        Some(Commands::New {  }) => {
            Scaffold::run().await?;
        }
        Some(Commands::Update { m }) => {
            Manager::update(m).await?;
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
        Some(Commands::Repo {  }) => {
            let config = Config::get()?;
            let git_account = match config.git.get_default_account() {
                Some(default_account) => {
                    println!("git account: {:?}", default_account);
                    default_account
                    
                },
                None => {
                    println!("No default git account");
                    let token: String = Dialog::input("Enter token", None, None, None);
                    GitAccount::new(GitProvider::GitHub, &token).await?
                }
            };
            let repo = GitRepo::get(&git_account, "cnctd").await?;
            println!("Repo: {:?}", repo);
            
        }
        None => {
            Scaffold::run().await?;
        }
    }
    Ok(())
}