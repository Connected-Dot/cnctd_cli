use cnctd::cnctd_bump::bump_project;

use crate::{Commands, scaffold::Scaffold, display_logo, config::Config};

use self::commands::config::{route_config_command, ConfigOptions};

pub mod commands;

pub async fn route_command(command: Option<Commands>) -> anyhow::Result<()> {
    match command {
        Some(Commands::Config {} ) => {
            display_logo("config", false);
            Config::launch_config_setup()
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
        None => {

        }
    }
    Ok(())
}