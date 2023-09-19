use std::fmt;
use cnctd::cnctd_dialogue::Dialog;
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::scaffold::module::ModuleScaffold;

use self::project::ProjectScaffold;

pub mod apps;
pub mod project;
pub mod module;

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, Default, PartialEq)]
pub enum MainOptions {
    #[default]
    LaunchNewProject,
    CreateModule,
    GetConfig,
    Exit,
}

impl fmt::Display for MainOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::LaunchNewProject => "Launch new project",
            Self::CreateModule => "Create module",
            Self::GetConfig => "Get config",
            Self::Exit => "Exit",
        };
        write!(f, "{}", display_str)
    }
}

pub struct Scaffold {}

impl Scaffold {
    pub async fn run() -> anyhow::Result<()> {

        loop {
            println!("\n{}\n","Welcome to cnctd Scaffold!".cyan().bold());

            let main_selection = Dialog::select::<MainOptions>("What would you like to do?", None, None, None);

            match main_selection {
                MainOptions::LaunchNewProject=> {
                    let mut project = ProjectScaffold::new();
                    loop {
                        println!("\n");
                        // Get the project name
                        project
                            .get_app_settings()
                            .set_name()
                            .set_project_dir()
                            .build().await?;

                        break;  // This will exit the inner loop and go back to the main menu
                    }
                },
                MainOptions::CreateModule => {
                    let mut module = ModuleScaffold::new();
                    loop {
                        println!("\n");

                        module 
                            .set_name()
                            .set_module_directory()
                            .build().await?;

                        break;
                    }
                }
                MainOptions::GetConfig => {
                    // Your logic for getting the config
                },
                MainOptions::Exit => {
                    break;
                },
            }
        }
        Ok(())
    }
}