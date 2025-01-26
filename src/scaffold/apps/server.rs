use std::{fmt, fs::create_dir_all, path::Path, env::set_current_dir};

use cnctd_cargo::{Cargo, Crate};
use cnctd_dialogue::Dialog;
use cnctd_go::Go;
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::scaffold::project::ProjectScaffold;

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, Default, PartialEq)]
pub enum ServerFlavor {
    #[default]
    GoWebRustServices,
    AllRust,
    AllGo,
}

impl fmt::Display for ServerFlavor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::GoWebRustServices => "Go webserver with Rust Services",
            Self::AllRust => "Rust all the way",
            Self::AllGo => "Go all the way",
        };
        write!(f, "{}", display_str)
    }
}


#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, PartialEq)]
pub enum ServerFeature {
    Redis,
    Aws,
    Database,
    Dockerize,
}


impl fmt::Display for ServerFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::Redis => "Redis",
            Self::Aws => "AWS",
            Self::Database => "Postgres DB",
            Self::Dockerize => "Dockerize"
        };
        write!(f, "{}", display_str)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct ServerApp {
    pub flavor: ServerFlavor,
    pub port: u16,
    pub features: Vec<ServerFeature>,
}

impl ServerApp {
    pub fn new() -> Self {
        Self {
            flavor: ServerFlavor::GoWebRustServices,
            port: 0,
            features: vec![],
        }
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub fn choose_server_options() -> Self {
        let mut server_app = Self::new();

        server_app
            .select_port()
            .select_flavor()
            .select_features();

        server_app

    }

    fn select_port(&mut self) -> &mut Self {
        self.port = Dialog::input("Enter the port number", Some(8181), None, None);
        
        self
    }
    

    fn select_flavor(&mut self) -> &mut Self {
        self.flavor = Dialog::select::<ServerFlavor>("Which flavor of server would you like?", None, None, None);

        self
    }
    

    fn select_features(&mut self) -> &mut Self {
        let default_features = Some(vec![ServerFeature::Dockerize]);
        let selected_features = Dialog::multi_select::<ServerFeature>("Which server features would you like to include?", default_features, None, None);
    
        self.features = selected_features;
        
        self
    }
    
    // #[async_recursion::async_recursion]
    pub async fn build(&self, project: &mut ProjectScaffold) -> anyhow::Result<()> {
        println!("building server");

        match self.flavor {
            ServerFlavor::GoWebRustServices => {
                let project_dir = format!("{}/{}", project.directory, project.name);
                let go_webserver_dir = Path::new("server/go_webserver");
                let rust_services_dir = Path::new("server/rust_services");

                set_current_dir(&project_dir)?;
                create_dir_all(&go_webserver_dir)?;
                create_dir_all(Path::new(&rust_services_dir))?;
                set_current_dir(&go_webserver_dir)?;
                Go::init(&project.name).await?;
                // Go::install_package(package_name)?;
            }
            ServerFlavor::AllRust => {}
            ServerFlavor::AllGo => {}
        }

        Ok(())
    }

    pub async fn add_base_crates() {
        let defaults = Self::base_defaults();
    
        for default in defaults {
            let crate_name = &default.to_rust_crate().name;
            match Cargo::install_crate(&crate_name).await {
                Ok(()) => println!("{}", format!("{} {}", &crate_name, "installed successfully".green())),
                Err(e) => println!("{}", e.to_string().red())
            }
        }
    }

    // fn rust_webserver_defaults() -> Vec<Crate> {
    //     vec![Crate::Warp]
    // }

    fn base_defaults() -> Vec<Crate> {
        vec![
            Crate::Tokio, 
            Crate::TokioStream,
            Crate::Dotenv, 
            Crate::Serde,
            Crate::SerdeJson,
            Crate::Anyhow,
            Crate::Futures,
        ]
    }

    // fn build_go_server

}
