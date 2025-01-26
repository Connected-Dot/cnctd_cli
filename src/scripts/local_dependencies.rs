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
        println!("Current directory: {}", current_dir.display());

        let temp_dir = current_dir.join("temp");

        // Create the temp directory
        if temp_dir.exists() {
            println!("Removing existing temp directory...");
            fs::remove_dir_all(&temp_dir)?;
        }
        println!("Creating temp directory...");
        fs::create_dir(&temp_dir)?;

        // Copy the main Cargo.toml to temp
        let main_cargo_path = current_dir.join("Cargo.toml");
        let temp_main_cargo_path = temp_dir.join("Cargo.toml");
        println!(
            "Copying main Cargo.toml from {} to {}...",
            main_cargo_path.display(),
            temp_main_cargo_path.display()
        );
        fs::copy(&main_cargo_path, &temp_main_cargo_path)?;

        // Track already processed dependencies to avoid duplication
        let mut processed_projects = HashSet::new();

        // Process the main Cargo.toml
        println!("Processing main Cargo.toml...");
        process_cargo_toml(
            &temp_main_cargo_path,
            &current_dir,
            &temp_dir,
            &mut processed_projects,
        )?;

        println!("All done!");
        Ok(())
    }
}

fn process_cargo_toml(
    cargo_toml_path: &Path,
    base_dir: &Path,
    temp_dir: &Path,
    processed_projects: &mut HashSet<PathBuf>,
) -> io::Result<()> {
    println!("Reading Cargo.toml: {}", cargo_toml_path.display());
    let content = fs::read_to_string(cargo_toml_path)?;
    let mut doc = content.parse::<DocumentMut>().expect("Failed to parse Cargo.toml");

    if let Some(deps) = doc.get_mut("dependencies").and_then(Item::as_table_like_mut) {
        println!("Found dependencies in {}: {:?}", cargo_toml_path.display(), deps.iter().collect::<Vec<_>>());
        for (name, item) in deps.iter_mut() {
            println!("Checking dependency: {}", name);
            if let Some(path) = get_dependency_path(item) {
                println!("Found local dependency at path: {}", path.display());
                let original_path = base_dir.join(&path);

                if !processed_projects.contains(&original_path) {
                    println!("Copying dependency: {}...", name);
                    processed_projects.insert(original_path.clone());

                    let name_str = name.to_string();
                    let dest_path = temp_dir.join(name_str);
                    fs::create_dir_all(&dest_path)?;
                    copy_dir(&original_path, &dest_path)?;

                    let sub_cargo_toml = dest_path.join("Cargo.toml");
                    println!("Processing subproject Cargo.toml: {}", sub_cargo_toml.display());
                    process_cargo_toml(
                        &sub_cargo_toml,
                        &original_path,
                        temp_dir,
                        processed_projects,
                    )?;
                }

                let new_path = format!("../{}", name);
                println!(
                    "Updating dependency path for {}: {}",
                    name, new_path
                );
                update_dependency_path(item, &new_path);
            } else {
                println!("No local path for dependency: {}", name);
            }
        }
    } else {
        println!("No dependencies found in {}", cargo_toml_path.display());
    }

    println!("Saving updated Cargo.toml: {}", cargo_toml_path.display());
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
    println!("Copying directory from {} to {}", src.display(), dest.display());
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dest.join(entry.file_name());
        if file_type.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir(&entry.path(), &dest_path)?;
        } else if file_type.is_file() {
            println!("Copying file: {}", entry.path().display());
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}
