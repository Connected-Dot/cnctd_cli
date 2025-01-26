use std::fmt;

use cnctd_dialogue::Dialog;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, PartialEq)]
pub enum DesktopFeature {
    Server,
    Redis,
    Aws,
}

impl fmt::Display for DesktopFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::Server => "Local webserver",
            Self::Redis => "Redis",
            Self::Aws => "AWS",
        };
        write!(f, "{}", display_str)
    }
}

impl DesktopFeature {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Server => "Local Server",
            Self::Redis => "Redis",
            Self::Aws => "AWS"
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct DesktopApp {
    features: Vec<DesktopFeature>
}

impl DesktopApp {
    pub fn new() -> Self {
        DesktopApp { features: vec![] }
    }

    pub fn choose_desktop_options() -> Self {
        let mut desktop_app = Self::new();

        let prompt = "Which desktop features would you like to include?";
        let selected_features = Dialog::multi_select::<DesktopFeature>(prompt, None, None, None);
        
        for feature in selected_features {
            desktop_app.features.push(feature);
        }
            
        desktop_app
    }
}

