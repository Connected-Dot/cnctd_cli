extern crate walkdir;
extern crate toml;
extern crate serde_json;
extern crate regex;

use walkdir::{WalkDir, DirEntry};
use std::fs::File;
use std::io::Read;
use toml::Value as TomlValue;
use serde_json::Value as JsonValue;
use regex::Regex;

pub fn print_project_versions(root_path: &str) -> std::io::Result<()> {
    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_entry(|e| !is_ignored(e))
    {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap_or_default();

        if path.is_file() {
            match file_name.to_str().unwrap() {
                // For Rust
                "Cargo.toml" => {
                    let mut file = File::open(path)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    let toml: TomlValue = contents.parse().unwrap();
                    if let Some(package) = toml.get("package") {
                        if let Some(name) = package.get("name").and_then(TomlValue::as_str) {
                            if let Some(version) = package.get("version").and_then(TomlValue::as_str) {
                                println!("Rust Project: {}, Version: {}", name, version);
                            }
                        }
                    }
                },
                // For npm
                "package.json" => {
                    let mut file = File::open(path)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    let json: JsonValue = serde_json::from_str(&contents).unwrap();
                    if let Some(name) = json["name"].as_str() {
                        if let Some(version) = json["version"].as_str() {
                            println!("NPM Project: {}, Version: {}", name, version);
                        }
                    }
                },
                // For Go
                "go.mod" => {
                    let mut file = File::open(path)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    let re = Regex::new(r"module\s+(?P<name>\S+)\s+go\s+(?P<version>\S+)").unwrap();
                    if let Some(caps) = re.captures(&contents) {
                        let name = &caps["name"];
                        let version = &caps["version"];
                        println!("Go Project: {}, Version: {}", name, version);
                    }
                },
                // For iOS
                file_name if file_name.ends_with(".xcodeproj") || file_name.ends_with(".xcworkspace") => {
                    println!("iOS Project: Located at {}", path.display());
                },
                // For Android
                "build.gradle" => {
                    let mut file = File::open(path)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    let re = Regex::new(r#"applicationId\s+['"](?P<name>[^"]+)['"]"#).unwrap();
                    if let Some(caps) = re.captures(&contents) {
                        let name = &caps["name"];
                        println!("Android Project: {}, Located at {}", name, path.display());
                    }
                },
                _ => {}
            }
        }
    }
    Ok(())
}


fn is_ignored(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s == "target" || s == "node_modules").unwrap_or(false)
}