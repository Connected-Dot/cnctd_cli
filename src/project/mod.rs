extern crate walkdir;
extern crate toml;
extern crate serde_json;
extern crate regex;

use walkdir::{WalkDir, DirEntry};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufRead};
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

pub fn update_rust_project_versions(root_path: &str) -> std::io::Result<()> {
    // Step 1: Collect all Rust project names and versions
    let mut project_versions: HashMap<String, String> = HashMap::new();
    
    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let file_name = path.file_name().unwrap_or_default();

        if path.is_file() && file_name.to_str().unwrap() == "Cargo.toml" {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let toml: TomlValue = contents.parse().unwrap();
            
            if let Some(package) = toml.get("package") {
                if let Some(name) = package.get("name").and_then(TomlValue::as_str) {
                    if let Some(version) = package.get("version").and_then(TomlValue::as_str) {
                        project_versions.insert(name.to_string(), version.to_string());
                    }
                }
            }
        }
    }

    // Step 2: Update dependencies in each Rust project
    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let file_name = path.file_name().unwrap_or_default();

        if path.is_file() && file_name.to_str().unwrap() == "Cargo.toml" {
            let temp_file_path = format!("{}.tmp", path.to_string_lossy());
            let mut temp_file = File::create(&temp_file_path)?;
            let file = File::open(&path)?;
            let reader = BufReader::new(file);

            let mut in_dependencies_section = false;

            for line in reader.lines() {
                let line = line?;
                if line.trim() == "[dependencies]" {
                    in_dependencies_section = true;
                } else if line.trim().starts_with('[') {
                    in_dependencies_section = false;
                }

                if in_dependencies_section {
                    let mut new_line = line.clone();
                    for (name, version) in &project_versions {
                        if line.contains(name) && line.contains("path") {
                            let version_line = format!("version = \"{}\"", version);
                            new_line = line.replacen("version = \"[^\"]+\"", &version_line, 1);
                        }
                    }
                    writeln!(temp_file, "{}", new_line)?;
                } else {
                    writeln!(temp_file, "{}", line)?;
                }
            }

            // Replace the original file with the temp file
            std::fs::rename(temp_file_path, &path)?;
        }
    }

    Ok(())
}

fn is_ignored(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s == "target" || s == "node_modules").unwrap_or(false)
}