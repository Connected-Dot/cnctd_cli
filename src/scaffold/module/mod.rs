use std::{env::current_dir, fmt};

use cnctd::cnctd_dialogue::Dialog;
use serde::{Deserialize, Serialize};

use self::{rust::RustModule, go::GoModule};

pub mod rust;
pub mod go;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModuleScaffold {
    pub name: String,
    pub directory: String,
    pub module: Module,
}

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, PartialEq)]
pub enum Module {
    Rust(RustModule),
    Go(GoModule)
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
            module: Self::get_mod()
        }
    }

    pub fn set_name(&mut self) -> &mut Self {
        let prompt = "Enter the module name";
        let project_name = Dialog::input(prompt, Some("new_module".to_string()), None, None);

        self.name = project_name;

        self
    }

    pub fn set_module_directory(&mut self) -> &mut Self {
        let prompt = "Enter the directory where you would like to build the module";
        let dir = Dialog::input(prompt, Some(self.directory.clone()), None, None);

        self.directory = dir;

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
        println!("module: {:?}", self);
        Ok(())
    }

}