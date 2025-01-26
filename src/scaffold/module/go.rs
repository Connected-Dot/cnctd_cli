use std::fmt;

use cnctd_dialogue::Dialog;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct GoModuleScaffold {
    features: Vec<GoModFeature>,
}

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, PartialEq)]
pub enum GoModFeature {
    Async,
    Tests,    
}

impl fmt::Display for GoModFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::Async => "Async",
            Self::Tests => "Tests",
        };
        write!(f, "{}", display_str)
    }
}

impl GoModuleScaffold {
    pub fn new() -> Self {
        Self {
            features: vec![]
        }
    }

    fn select_features(&mut self) -> &mut Self {
        let default_features = Some(vec![GoModFeature::Async, GoModFeature::Tests]);
        let selected_features = Dialog::multi_select::<GoModFeature>("Pick your module options?", default_features, None, None);
    
        self.features = selected_features;
        
        self
    }

    pub fn choose_options() -> Self {
        let mut rust_module = Self::new();
        rust_module.select_features();
        
        rust_module
    }

    
}