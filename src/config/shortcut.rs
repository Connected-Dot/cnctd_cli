use cnctd_dialogue::Dialog;
use cnctd_shell::Shell;
use colored::{Colorize, Color};
use serde::{Deserialize, Serialize};

use super::Config;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Shortcut {
    pub name: String,
    pub command: String,
}

impl Shortcut {
    pub fn add(shortcuts: &mut Vec<Shortcut>) {
        let name: String = Dialog::input("Enter name", None, None, None);
        let command = Dialog::input("Enter command", None, None, None);
        let shortcut = Shortcut { name: name.clone(), command };
        if shortcuts.iter().any(|shortcut| shortcut.name == name) {
            let prompt = "Shortcut exists. Replace?";
            let decision = Dialog::select_str(prompt, &vec!["Yes", "No"], None, Some(Color::Yellow), None);
            match &*decision {
                "Yes" => {
                    shortcuts.push(shortcut);
                }
                &_ => {

                }
            }
        } else {
            shortcuts.push(shortcut);
        }
    }

    pub fn remove(shortcuts: &mut Vec<Shortcut>) {
        let mut shortcut_names: Vec<&str> = vec![];
        for shortcut in shortcuts.iter() {
            shortcut_names.push(&shortcut.name);
        }
        let prompt = "which shortcut would you like to remove?";
        let decision = Dialog::select_str(prompt, &shortcut_names, None, None, None);
        
        let index_to_remove = shortcuts.iter().position(|shortcut| shortcut.name == decision);
        
        if let Some(index) = index_to_remove {
            shortcuts.remove(index);
            println!("Removed Shortcut: {}", decision);
        }
    }

    pub fn display_all(shortcuts: &mut Vec<Shortcut>) {
        if shortcuts.is_empty() { println!("{}\n", "No shortcuts configured".yellow()) }
        for shortcut in shortcuts {
            println!("{}: {}\n", shortcut.name.blue(), shortcut.command);
        }
    }

    pub async fn execute(name: &str) -> anyhow::Result<()> {
        let config = Config::get()?;

        if let Some(index) = config.shortcuts.iter().position(|shortcut| shortcut.name == name) {
            let command = &config.shortcuts[index].command;
            Shell::run(&command, true).await?;
        } else {
            println!("{}", format!("No shortcut with name: {}", name.italic()).yellow())
        }

        Ok(())
    }
}