use std::fmt;

use cnctd::cnctd_dialogue::Dialog;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, EnumIter)]
pub enum WebFeature {
    SSG,
    FontAwesome,
    Wasm,
}

impl fmt::Display for WebFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::SSG => "Static-Site Generator",
            Self::FontAwesome => "FontAwesome",
            Self::Wasm => "Wasm",
        };
        write!(f, "{}", display_str)
    }
}



#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct WebApp {
    features: Vec<WebFeature>
}

impl WebApp {
    pub fn new() -> Self {
        Self {
            features: vec![],
        }
    }
    pub fn choose_webapp_options() -> Self {
        let mut webapp = Self::new();
        
        let prompt = "Which frontend features would you like to include?";
        let selected_features = Dialog::multi_select::<WebFeature>(prompt, None, None, None);
            for feature in selected_features {
                match feature {
                    WebFeature::SSG => {
                        webapp.features.push(WebFeature::SSG)
                    },
                    WebFeature::FontAwesome => {
                        webapp.features.push(WebFeature::FontAwesome)
                    },
                    WebFeature::Wasm => {
                        webapp.features.push(WebFeature::Wasm)
                    }
                }
            }

        webapp
    }
}