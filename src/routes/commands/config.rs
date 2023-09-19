use crate::config::Config;

pub struct ConfigOptions {
    pub github_token: Option<String>,
    pub iphone: Option<String>,
    pub s1: Option<String>,
    pub s2: Option<String>,
    pub s3: Option<String>,
}


pub fn route_config_command(config: ConfigOptions) {
    match config.github_token {
        Some(token) => Config::set_git_token(&token),
        None => {}
    }
    match config.iphone {
        Some(token) => Config::set_git_token(&token),
        None => {}
    }
    match config.s1 {
        Some(shortcut) => Config::set_shortcut(1, &shortcut),
        None => {}
    }
    match config.s2 {
        Some(shortcut) => Config::set_shortcut(2, &shortcut),
        None => {}
    }
    match config.s3 {
        Some(shortcut) => Config::set_shortcut(3, &shortcut),
        None => {}
    }
}