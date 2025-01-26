use std::{env::current_dir, path::Path, fs::create_dir_all, fmt};
use cnctd_dialogue::Dialog;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use super::apps::{App, server::ServerApp, web::WebApp, desktop::DesktopApp, ios::IosApp, android::AndroidApp};


#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, Default, PartialEq)]
pub enum DirectoryFix {
        #[default]
    ChangeDirectory,
    ChangeProjectName,
    StartOver,
}

impl fmt::Display for DirectoryFix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::ChangeDirectory => "Change directory",
            Self::ChangeProjectName => "Change project name",
            Self::StartOver => "Start over",
        };
        write!(f, "{}", display_str)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectScaffold {
    pub name: String,
    pub directory: String,
    pub apps: Vec<App>,
}

impl ProjectScaffold {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            directory: current_dir().unwrap().to_string_lossy().to_string(),
            apps: Vec::new(),
        }
    }

    pub fn set_name(&mut self) -> &mut Self {
        let prompt = "Enter the project name";
        let project_name = Dialog::input(prompt, Some("new_project".to_string()), None, None);

        self.name = project_name;

        self
    }

    pub fn set_project_dir(&mut self) -> &mut Self {
        let prompt = "Enter the directory where you would like to build the project";
        let dir = Dialog::input(prompt, Some(self.directory.clone()), None, None);

        self.directory = dir;

        self
    }

    pub fn get_app_settings(&mut self) -> &mut Self {
        let prompt = "Which apps would you like to include?";
        let app_selections = Dialog::multi_select::<App>(prompt, None, None, None);
        
        let mut new_apps: Vec<App> = app_selections.iter().map(|app| {
            match app {
                App::Server(_) => App::Server(ServerApp::choose_server_options()),
                App::Web(_) => App::Web(WebApp::choose_webapp_options()),
                App::Desktop(_) => App::Desktop(DesktopApp::choose_desktop_options()),
                App::Ios(_) => App::Ios(IosApp { /* fields */ }),
                App::Android(_) => App::Android(AndroidApp { /* fields */ }),
            }
        }).collect();
        
        if new_apps.iter().any(|app| matches!(app, App::Web(_) | App::Ios(_) | App::Android(_) | App::Desktop(_))) {
            new_apps.push(App::Web(WebApp::choose_webapp_options()));
        }
        
        self.apps.extend(new_apps);        

        self
    }
    
    pub async fn build(&mut self) -> anyhow::Result<()> {
        println!("{}", "Starting build process".green());
        println!("{}", "Creating project directory if it does not exist");
        let dir = format!("{}/{}", &self.directory, &self.name.to_lowercase().replace(" ", "_").replace("-", "_"));
        let path = Path::new(&dir);

        if !path.exists() { create_dir_all(&path)? } else { self.fix_directory_issue().await }
        
        let apps = self.apps.clone();
        
        for app in apps {
            match app {
                App::Server(server_app) => {
                    server_app.build(self).await?;
                    // Do something with server_app, which is of type &ServerApp
                },
                App::Web(_web_app) => {
                    // Do something with web_app, which is of type &WebApp
                },
                App::Desktop(_desktop_app) => {
                    // Do something with desktop_app, which is of type &DesktopApp
                },
                App::Ios(_ios_app) => {
                    // Do something with ios_app, which is of type &IosApp
                },
                App::Android(_android_app) => {
                    // Do something with android_app, which is of type &AndroidApp
                },
                // Add other cases as needed
            }
        }

        Ok(())
    }

    #[async_recursion::async_recursion]
    pub async fn fix_directory_issue(&mut self) {
        println!("{}","Directory already exists".yellow());
        let prompt = "How would you like to proceed?";
        let selection = Dialog::select::<DirectoryFix>(prompt, None, None, None);

        match selection {
            DirectoryFix::ChangeDirectory => {
                self
                    .set_project_dir()
                    .build()
                    .await
                    .unwrap();
            },
            DirectoryFix::ChangeProjectName => {
                self
                    .set_name()
                    .build()
                    .await
                    .unwrap();
            },
            _ => {},
        }
    }
}