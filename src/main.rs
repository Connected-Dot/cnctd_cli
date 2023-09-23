#[macro_use]
extern crate strum_macros;
extern crate strum;

use std::{thread, time::Duration, io::{stdout, Write}};

use clap::{Parser, Subcommand};
use colored::*;
use config::Config;
use crossterm::{execute, terminal::{Clear, ClearType}, style::Stylize};
use routes::route_command;
use tokio;
use dotenv::dotenv;
use figlet_rs::FIGfont;

pub mod project;
pub mod routes;
pub mod scaffold;
pub mod config;

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

pub fn get_logo(word: &str) -> String {
    let standard_font = FIGfont::standard().unwrap();
    standard_font.convert(word).unwrap().to_string().cyan().to_string()
}

pub fn print_separator(length: u8, animate: bool) {
    println!("");
    for _i in 1..length {
        print!("-");
        std::io::stdout().flush().unwrap();
        if animate { thread::sleep(Duration::from_millis(10)) }
    }
    println!("");
}