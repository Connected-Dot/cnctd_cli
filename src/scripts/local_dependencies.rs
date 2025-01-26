use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use toml_edit::{DocumentMut, Item, Value};

pub struct LocalDependencies;

impl LocalDependencies {
    pub async fn run() -> anyhow::Result<()> {
        let current_dir = env::current_dir()?;
        println!("Current directory: {}", current_dir.display());

        let temp_dir = current_dir.join("temp");

        // Step 1: Create temp directory
        if temp_dir.exists() {
            println!("Removing existing temp directory...");
            fs::remove_dir_all(&temp_dir)?;
        }
        println!("Creating temp directory...");
        fs::create_dir(&temp_dir)?;

        let main_cargo_path = current_dir.join("Cargo.toml");
        let temp_main_cargo_path = temp_dir.join("Cargo.toml");

        // Step 2: Copy main Cargo.toml to temp
        println!(
            "Copying main Cargo.toml from {} to {}...",
            main_cargo_path.display(),
            temp_main_cargo_path.display()
        );
        fs::copy(&main_cargo_path, &temp_main_cargo_path)?;

        let mut processed_projects = HashSet::new();

        // Step 3: Process main Cargo.toml to discover and copy submodules
        println!("Discovering and copying submodules...");
        discover_and_copy_submodules(
            &temp_main_cargo_path,
            &current_dir,
            &temp_dir,
            &mut processed_projects,
        )?;

        // Step 4: Update paths in the main Cargo.toml
        println!("Updating paths in the main Cargo.toml...");
        update_main_cargo_paths(&temp_main_cargo_path)?;

        println!("All done!");
        Ok(())
    }
}

// Function to discover submodules and copy them
fn discover_and_copy_submodules(
    cargo_toml_path: &Path,
    original_base_dir: &Path,
    temp_dir: &Path,
    processed_projects: &mut HashSet<PathBuf>,
) -> io::Result<()> {
    println!("Reading Cargo.toml: {}", cargo_toml_path.display());
    let content = fs::read_to_string(cargo_toml_path)?;
    let doc = content.parse::<DocumentMut>().expect("Failed to parse Cargo.toml");

    if let Some(deps) = doc.get("dependencies").and_then(Item::as_table_like) {
        for (name, item) in deps.iter() {
            if let Some(path) = get_dependency_path(item) {
                let original_path = original_base_dir.join(&path);

                if !processed_projects.contains(&original_path) {
                    println!("Copying dependency: {}...", name);
                    processed_projects.insert(original_path.clone());

                    let dest_path = temp_dir.join(name.to_string());
                    fs::create_dir_all(&dest_path)?;
                    copy_dir(&original_path, &dest_path)?;

                    // Process submodule Cargo.toml recursively
                    let sub_cargo_toml = dest_path.join("Cargo.toml");
                    discover_and_copy_submodules(&sub_cargo_toml, &original_path, temp_dir, processed_projects)?;
                }
            }
        }
    }
    Ok(())
}

// Function to update paths in the main Cargo.toml
fn update_main_cargo_paths(cargo_toml_path: &Path) -> io::Result<()> {
    println!("Updating paths in: {}", cargo_toml_path.display());
    let content = fs::read_to_string(cargo_toml_path)?;
    let mut doc = content.parse::<DocumentMut>().expect("Failed to parse Cargo.toml");

    if let Some(deps) = doc.get_mut("dependencies").and_then(Item::as_table_like_mut) {
        for (name, item) in deps.iter_mut() {
            if let Some(path) = get_dependency_path(item) {
                let new_path = format!("temp/{}", name);
                println!("Updating path for {}: {}", name, new_path);
                update_dependency_path(item, &new_path);
            }
        }
    }

    let mut file = File::create(cargo_toml_path)?;
    file.write_all(doc.to_string().as_bytes())?;
    println!("Paths updated in: {}", cargo_toml_path.display());
    Ok(())
}

fn get_dependency_path(item: &Item) -> Option<PathBuf> {
    match item {
        Item::Table(table) => table
            .get("path")
            .and_then(Item::as_value)
            .and_then(Value::as_str)
            .map(PathBuf::from),
        Item::Value(Value::InlineTable(table)) => table
            .get("path")
            .and_then(Value::as_str)
            .map(PathBuf::from),
        _ => None,
    }
}

fn update_dependency_path(item: &mut Item, new_path: &str) {
    match item {
        Item::Table(table) => {
            if let Some(path) = table.get_mut("path").and_then(Item::as_value_mut) {
                *path = Value::from(new_path);
            }
        }
        Item::Value(Value::InlineTable(table)) => {
            if let Some(path) = table.get_mut("path") {
                *path = Value::from(new_path);
            }
        }
        _ => {}
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
