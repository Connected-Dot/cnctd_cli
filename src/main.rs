#[macro_use]
extern crate strum_macros;
extern crate strum;

use clap::{Parser, Subcommand};
use cnctd::cnctd_utils::get_logo;
use routes::route_command;
use tokio;
use dotenv::dotenv;

pub mod project;
pub mod routes;
pub mod scaffold;
pub mod config;
pub mod manager;
pub mod scripts;

#[derive(Parser)]
#[command(author, version, about = get_logo("cnctd"), long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    #[arg(short, long)]
    update: Option<String>,
}


#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configure Settings
    Config {
        
    },
    /// Start something new
    New {
        
    },

    /// Update git repo and publish module
    Update {
        #[arg(short, long)]
        m: Option<String>,
    },

    /// Get versions of all apps in current dir
    Versions {
        /// Optional directory
        #[arg(short, long)]
        d: Option<String>,
    },

    /// Execute named shortcut
    S {
        #[command()]
        name: String,
    },

    /// Bump Project
    Bump {
        #[command()]
        version_part: Option<String>,
    },

    /// Get Repo
    Repo {
        
    },

    /// Update Workspace
    Workspace {

    },

    /// Manage Submodule
    Submodule {

    },

    Scripts {

    },
}

#[tokio::main]
async fn main() {    
    dotenv().ok();
    let cli = Cli::parse();
    match route_command(cli.command).await {
        Ok(()) => {}
        Err(e) => println!("Error: {}", e)
    }
}

