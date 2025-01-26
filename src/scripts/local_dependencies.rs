use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use toml_edit::{DocumentMut, Item, Value};

pub struct LocalDependencies;

impl LocalDependencies {
    pub fn run() -> io::Result<()> {
        // Get the current directory
        let current_dir = env::current_dir()?;
        let temp_dir = current_dir.join("temp");
    
        // Create the temp directory
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        fs::create_dir(&temp_dir)?;
    
        // Copy the main Cargo.toml to temp
        let main_cargo_path = current_dir.join("Cargo.toml");
        let temp_main_cargo_path = temp_dir.join("Cargo.toml");
        fs::copy(&main_cargo_path, &temp_main_cargo_path)?;
    
        // Track already processed dependencies to avoid duplication
        let mut processed_projects = HashSet::new();
    
        // Process the main Cargo.toml
        process_cargo_toml(&temp_main_cargo_path, &current_dir, &temp_dir, &mut processed_projects)?;
    
        Ok(())
    }
}

fn process_cargo_toml(
    cargo_toml_path: &Path,
    base_dir: &Path,
    temp_dir: &Path,
    processed_projects: &mut HashSet<PathBuf>,
) -> io::Result<()> {
    // Read and parse the Cargo.toml
    let content = fs::read_to_string(cargo_toml_path)?;
    let mut doc = content.parse::<DocumentMut>().expect("Failed to parse Cargo.toml");

    // Get dependencies and update paths
    if let Some(deps) = doc.get_mut("dependencies").and_then(Item::as_table_like_mut) {
        for (name, item) in deps.iter_mut() {
            if let Some(path) = get_dependency_path(item) {
                let original_path = base_dir.join(&path);
                if !processed_projects.contains(&original_path) {
                    // Mark the project as processed
                    processed_projects.insert(original_path.clone());

                    // Copy the project to the temp directory
                    let dest_path = temp_dir.join(name.to_string());
                    fs::create_dir_all(&dest_path)?;
                    copy_dir(&original_path, &dest_path)?;

                    // Process the copied Cargo.toml
                    let sub_cargo_toml = dest_path.join("Cargo.toml");
                    process_cargo_toml(&sub_cargo_toml, &original_path, temp_dir, processed_projects)?;
                }

                // Update the path to be relative to the temp directory
                let new_path = format!("../{}", name);
                update_dependency_path(item, &new_path);
            }
        }
    }

    // Save the updated Cargo.toml
    let mut file = File::create(cargo_toml_path)?;
    file.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

fn get_dependency_path(item: &Item) -> Option<PathBuf> {
    if let Item::Table(table) = item {
        if let Some(Value::String(path)) = table.get("path").and_then(Item::as_value) {
            return Some(PathBuf::from(path.value()));
        }
    }
    None
}

fn update_dependency_path(item: &mut Item, new_path: &str) {
    if let Item::Table(table) = item {
        if let Some(path) = table.get_mut("path").and_then(Item::as_value_mut) {
            *path = Value::from(new_path);
        }
    }
}

fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dest.join(entry.file_name());
        if file_type.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir(&entry.path(), &dest_path)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}
