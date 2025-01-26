use std::{env::current_dir, path::Path};
use cnctd_bump::bump_project;
use cnctd_cargo::Cargo;
use cnctd_dialogue::Dialog;
use cnctd_git::{account::GitAccount, repo::GitRepo, GitProvider};
use cnctd_utils::get_relative_path;
use colored::Colorize;

use crate::{config::{shortcut::Shortcut, Config}, manager::Manager, project::print_project_versions, scaffold::Scaffold, scripts::Scripts, Commands};

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
        Some(Commands::Workspace {  }) => {
            let mut path = current_dir()?;
            path.push("Cargo.toml");
            let members = Cargo::get_workspace_members(&path)?;

            println!("Members: {:?}", members);

            if members.len() > 0 {
                for member in members {
                    let cargo_path = format!("{}/Cargo.toml", member);
                    let cargo_path = Path::new(&cargo_path);
                    let local_deps = Cargo::get_local_dependencies(cargo_path).await?;
                    println!("local_deps: {:?}", local_deps);
                }
            } else {
                Cargo::get_local_dependencies(&path).await?;
            }
        }
        Some(Commands::Submodule {  }) => {
            let mut path = current_dir()?;
            let project_dir = &path.as_mut_os_str().to_str().unwrap().to_string();
            let parent_dir;
            let repo_url = Cargo::get_package_repo(project_dir)?;
            let name = Cargo::get_package_name(project_dir)?;

            println!("Name: {}\nRepo URL: {}", name, repo_url);

            match GitRepo::find_git_root(&path) {
                Some(parent_path) => {
                    parent_dir = parent_path.into_os_string().into_string().unwrap();
                    let prompt = format!(
                        "Project is inside larger project directory: {}\nWould you like to initialize it as a submodule?", 
                        parent_dir
                    );
                    let decision = Dialog::select_str(&prompt, &vec!["Yes", "No"], None, None, None);
                    match &*decision {
                        "Yes" => {
                            let relative_path = get_relative_path(Path::new(&parent_dir), Path::new(&project_dir)).unwrap();
                            println!("relative path: {}", relative_path);
                            GitRepo::add_submodule(&parent_dir, &repo_url, &relative_path)?;
                            println!("{}", "Added submodule".green());
                        }
                        &_ => {}
                    }
                }
                None => {
                    println!("Project not inside larger project directory")
                }
            }
        }
        Some(Commands::Scripts {}) => {
            Scripts::launch_scripts_menu().await?;
        }
        None => {
            Scaffold::run().await?;
        }
    }
    Ok(())
}