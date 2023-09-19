use std::env::current_dir;

use cnctd::cnctd_bump::bump_project;

use crate::{Commands, scaffold::Scaffold, display_logo, project::print_project_versions};

use self::commands::config::{route_config_command, ConfigOptions};

pub mod commands;

pub async fn route_command(command: Option<Commands>) -> anyhow::Result<()> {
    match command {
        Some(Commands::Config { 
            github_token, 
            iphone,
            s1, 
            s2, 
            s3 
        }) => {
            let config_options = ConfigOptions { github_token, iphone, s1, s2, s3 };
            route_config_command(config_options);
        }
        Some(Commands::New {  }) => {
            display_logo("cnctd.", true);
            Scaffold::run().await?;
        }
        Some(Commands::Update {  }) => {
            
        }
        Some(Commands::S1 {  }) => {}
        Some(Commands::S2 {  }) => {}
        Some(Commands::S3 {  }) => {}
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

        }
    }
    Ok(())
}