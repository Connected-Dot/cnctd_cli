use serde::{Deserialize, Serialize};

use self::{server::ServerApp, web::WebApp, desktop::DesktopApp, ios::IosApp, android::AndroidApp};

pub mod android;
pub mod cli;
pub mod desktop;
pub mod ios;
pub mod web;
pub mod server;

#[derive(Debug, Deserialize, Serialize, Clone, Display, EnumIter, PartialEq)]
pub enum App {
    Server(ServerApp),
    Web(WebApp),
    Desktop(DesktopApp),
    // Cli(CliApp),  // Commented out for now
    Ios(IosApp),
    Android(AndroidApp),
}
