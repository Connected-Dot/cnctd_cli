#[macro_use]
extern crate strum_macros;
extern crate strum;

use std::{thread, time::Duration, io::stdout};

use clap::{Parser, Subcommand};
use colored::*;
use config::Config;
use crossterm::{execute, terminal::{Clear, ClearType}};
use routes::route_command;
use tokio;
use dotenv::dotenv;
use figlet_rs::FIGfont;

pub mod routes;
pub mod scaffold;
pub mod config;

#[derive(Parser)]
#[command(author, version, about = get_about(), long_about = None, arg_required_else_help = true)]
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

    },

    #[command(about = get_shortcut_about(1))]
    S1 {

    },
    #[command(about = get_shortcut_about(2))]
    S2 {

    },
    #[command(about = get_shortcut_about(3))]
    S3 {

    },

    /// Bump Project
    Bump {
        #[command()]
        version_part: Option<String>,
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

fn get_about() -> String {

    match Config::get() {
        Some(conf) => {
            let iphone = match conf.iphone {
                Some(id) => id.green(),
                None => "Not set. Run cnctd config -i <DEVICE_ID> to set".yellow()
            };
            format!("iPhone: {} Run cnctd config -i <DEVICE_ID> to change", iphone)
        }
        None => format!("{}", "GitHub Token not set. Run cnctd config -g <TOKEN>".yellow()).into()
    }
}

fn get_shortcut_about(id: u8) -> String {
    match Config::get_shortcut(id) {
        Some(shortcut) => format!("Shortcut {} set: {} | Run cnctd config --s{} <SHELL_COMMAND> to change",id, shortcut.green(), id),
        None => format!("Shortcut {} not set. Run cnctd config --s{} <SHELL_COMMAND> to set", id, id)
    }
}

pub fn get_exe_dir() -> String {
    std::env::current_exe().unwrap().to_str().unwrap().to_string()
}

pub fn clear_terminal() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

pub fn display_logo(word: &str, animate: bool) {
    let standard_font = FIGfont::standard().unwrap();
    let mut partial_word = String::new();

    for ch in word.chars() {
        partial_word.push(ch);
        let figure = standard_font.convert(&partial_word).unwrap();
        let logo = figure.to_string().cyan().to_string();

        clear_terminal();
        println!("{}", logo);

        if animate {
            thread::sleep(Duration::from_millis(100));
        }
    }
}
