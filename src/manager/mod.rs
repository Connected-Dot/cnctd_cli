use std::{env::current_dir, path::Path};

use cnctd_bump::bump_project;
use cnctd_cargo::Cargo;
use cnctd_dialogue::Dialog;
use cnctd_git::{api::local::Branch, repo::{GitRepo, ProjectType}};
use colored::Colorize;

use crate::config::Config;

pub mod rust;

pub struct Manager;


impl Manager {
    pub async fn update(message: Option<String>) -> anyhow::Result<()> {
        let message = match message {
            Some(message) => message,
            None => {
                Dialog::input::<String>(
                    "Enter message", 
                    Some("no message".to_string()),
                    None,
                    None,
                )
            }
        };

        bump_project("patch").await?;
        
        let decision = Dialog::select_str(
            "publish?", 
            &vec!["Yes", "No"], 
            None, 
            None, 
            None
        );
        
        let config = Config::get()?;
        let token = config.git.get_default_account().unwrap().token;
        
        let current_path = std::env::current_dir()?;
        let git_path = GitRepo::find_git_root(&current_path).unwrap_or(current_dir()?).as_mut_os_string().to_str().unwrap().to_string();

        GitRepo::update(
            &git_path, 
            &message, 
            Branch::Main, 
            Branch::Main, 
            &token
        )?;
        println!("{}", "Successfully updated repo".green());
        
        match &*decision {
            "Yes" => {
                Self::publish().await?;
            }
            &_ => {}
        }
    
        Ok(())
    
    }

    pub fn determine_project_type() -> anyhow::Result<ProjectType> {
        if Path::new("package.json").exists() {
            Ok(ProjectType::Node)
        } else if Path::new("go.mod").exists() {
            Ok(ProjectType::Go)
        } else if Path::new("Cargo.toml").exists() {
            Ok(ProjectType::Rust)
        } else {
            Err(anyhow::anyhow!("Not a supported project type"))
        }
    }

    pub async fn publish() -> anyhow::Result<()> {
        if Path::new("Cargo.toml").exists() {
            let project_dir = current_dir()?.as_path().to_str().unwrap().to_string();
            Cargo::publish_crate(&project_dir).await?;
            println!("{}", "Crate successfully published".green());
            
            Ok(())
        } 
        // else if Path::new("package.json").exists() {
        //     Ok(ProjectType::Node)
        // } 
        // else if Path::new("go.mod").exists() {
        //     Ok(ProjectType::Go)
        // } 
        else  {
            Err(anyhow::anyhow!("Not a supported project type"))
        }
    }
}