use std::fmt;

use async_recursion::async_recursion;
use cnctd_dialogue::Dialog;
use cnctd_utils::display_logo;
use local_dependencies::LocalDependencies;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

pub mod local_dependencies;

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, Default, PartialEq)]
enum MainOptions {
    #[default]
    LocalDependencies,
    Blank,
    Back,
    Exit,
}

impl fmt::Display for MainOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::LocalDependencies => "Local Dependencies",
            Self::Blank => "- - - -",
            Self::Back => "Back",
            Self::Exit => "Exit",
        };
        write!(f, "{}", display_str)
    }
}

pub struct Scripts;

impl Scripts {
    #[async_recursion]
    pub async fn launch_scripts_menu() -> anyhow::Result<()> {
        display_logo("scripts", false);
        let prompt = "Which script would you like to run?";
        let selected_option = Dialog::select::<MainOptions>(prompt, None, None, None);
    
        match selected_option {
            MainOptions::LocalDependencies => Self::local_dependencies(),
            MainOptions::Blank => Self::launch_scripts_menu().await?,
            MainOptions::Back => {},
            MainOptions::Exit => std::process::exit(0),
            
        }
        Ok(())
    }

    pub fn local_dependencies() {
        LocalDependencies::run().unwrap();
    }
}