use crate::{Commands, scaffold::Scaffold, display_logo};

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
        None => {

        }
    }
    Ok(())
}