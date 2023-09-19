use std::fmt;

use cnctd::cnctd_dialogue::Dialog;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct RustModule {
    features: Vec<RustModFeature>,
}

#[derive(Debug, Deserialize, Serialize, Clone, EnumIter, PartialEq)]
pub enum RustModFeature {
    Async,
    Tests,    
}

impl fmt::Display for RustModFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Self::Async => "Async",
            Self::Tests => "Tests",
        };
        write!(f, "{}", display_str)
    }
}

impl RustModule {
    pub fn new() -> Self {
        Self {
            features: vec![]
        }
    }

    fn select_features(&mut self) -> &mut Self {
        let default_features = Some(vec![RustModFeature::Async, RustModFeature::Tests]);
        let selected_features = Dialog::multi_select::<RustModFeature>("Pick your crate options?", default_features, None, None);
    
        self.features = selected_features;
        
        self
    }

    pub fn choose_options() -> Self {
        let mut rust_module = Self::new();
        rust_module.select_features();
        
        rust_module
    }

    
}