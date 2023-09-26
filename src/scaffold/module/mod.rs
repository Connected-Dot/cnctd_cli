use std::{env::current_dir, fmt, path::Path};

use cnctd::{cnctd_dialogue::Dialog, cnctd_cargo::{Cargo, CrateType}, cnctd_git::repo::{GitRepo, ProjectType}};
use cnctd_utils::get_relative_path;
use colored::Colorize;
use serde::{Deserialize, Serialize};



use self::{rust::RustModuleScaffold, go::GoModuleScaffold};

use super::Scaffold;

pub mod rust;
pub mod go;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModuleScaffold {
    pub name: String,
    pub directory: String,
    pub module: Module,
    pub description: String,
    pub is_private: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, PartialEq)]
pub enum Module {
    Rust(RustModuleScaffold),
    Go(GoModuleScaffold)
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::Rust(_) => "Rust",
            Self::Go(_) => "Go",
        };
        write!(f, "{}", display_str)
    }
}

impl ModuleScaffold {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            directory: current_dir().unwrap().to_string_lossy().to_string(),
            description: String::new(),
            module: Self::get_mod(),
            is_private: true,
        }
    }

    pub fn set_name(&mut self) -> &mut Self {
        let prompt = "Enter the module name";
        let project_name = Dialog::input(prompt, Some("new_module".to_string()), None, None);

        self.name = project_name;

        self
    }


    pub fn set_description(&mut self) -> &mut Self {
        let prompt = "Enter the description";
        let project_description = Dialog::input(prompt, Some("scaffolded by cnctd".to_string()), None, None);

        self.description = project_description;

        self
    }

    pub fn set_module_directory(&mut self) -> &mut Self {
        let prompt = "Enter the directory where you would like to build the module";
        let dir = Dialog::input(prompt, Some(self.directory.clone()), None, None);

        self.directory = dir;

        self
    }

    pub fn set_is_private(&mut self) -> &mut Self {
        let prompt = "Private Repo?";
        let options = vec!["Yes", "No"];
        let is_private = Dialog::select_str(prompt, &options, None, None, None);

        self.is_private = if is_private == "Yes" { true } else { false };

        self
    }

    pub fn get_mod() -> Module{
        let prompt = "Which type of module would you like to build?";
        let module = Dialog::select::<Module>(prompt, None, None, None);
        match module {
            Module::Go(go_module) => {
                Module::Go(go_module)
            }
            Module::Rust(rust_module) => {
                Module::Rust(rust_module)
            }   
        }
    }

    pub async fn build(&mut self) -> anyhow::Result<()> {
        let project_dir = format!("{}/{}", &self.directory, &self.name);
        let selected_account = Scaffold::select_git_account().await?;
          
        let repo = GitRepo::new(&selected_account, &self.name, Some(&self.description), self.is_private).await?;
        println!("{}", format!("Created repo at {}", repo.html_url).green());

        match &self.module {
            Module::Rust(_module_scaffold) => {
                let author = Scaffold::select_authors().await?;
                let license = Scaffold::select_license()?;
                Cargo::init(&project_dir, CrateType::Module).await?;
                println!("{}", "Initialized crate".green());
                Cargo::update_cargo_toml(
                    author, 
                    &self.description, 
                    &repo.html_url, 
                    &license,
                    CrateType::Module
                ).await?;
                println!("{}", "Updated Cargo.toml".green());
                GitRepo::init(&project_dir)?;
                println!("{}", "Initialized Repo".green());
                GitRepo::add_gitignore(&project_dir, ProjectType::Rust)?;
                println!("{}", "Added gitignore".green());
                GitRepo::remote_add_origin(&project_dir, &repo.html_url)?;
                println!("{}", "Added remote origin".green());
                GitRepo::first_commit(&project_dir, &selected_account.token)?;
                println!("{}", "Sent first commit".green());
                let prompt = "Module created successfully. Publish to crates.io?";
                let decision = Dialog::select_str(prompt, &vec!["Yes", "No"], None, None, None);
                match &*decision {
                    "Yes" => {
                        Cargo::publish_crate(&project_dir).await?;
                        println!("{}", "Crate successfully published!".green());
                    }
                    &_ => {}
                }
                let parent_dir;
                match GitRepo::find_git_root(Path::new(&project_dir)) {
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
                                GitRepo::add_submodule(&parent_dir, &repo.html_url, &relative_path)?;
                                println!("{}", "Added submodule".green());
                            }
                            &_ => {}
                        }
                    }
                    None => {}
                }

            }
            Module::Go(_module_scaffold) => {}

            
        }
        

        Ok(())
    }

}