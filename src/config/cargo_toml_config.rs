use std::{thread, time::Duration};

use cnctd_cargo::cargo_toml::Author;
use cnctd_dialogue::Dialog;
use colored::Colorize;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct CargoTomlConfig {
    pub authors: Option<Vec<Author>>,
    pub default_author: Option<String>,
    pub default_license: Option<String>,
}


impl CargoTomlConfig {
    pub fn add_author(&mut self) -> anyhow::Result<Author> {
        let name = Dialog::input("Enter name", None, None, None);
        let email: String = Dialog::input("Enter email", None, None, None);
        let organization = Dialog::input("Enter organization", None, None, None);
        let author = Author { name, organization, email: email.clone() };
        
        let email_clone = email.clone();
    
        match &mut self.authors {
            Some(authors) => {
                if authors.iter().any(|auth| auth.email == email) {
                    println!("{}", "This account already exists.".yellow());
                    thread::sleep(Duration::from_secs(2));
                } else {
                    authors.push(author.clone());
                    if authors.len() == 1 { self.default_author = Some(email_clone) }
                }
                
            }
            None => {
                let authors = Some(vec![author.clone()]);
                self.authors = authors;
                self.default_author = Some(email_clone)
            }
        }
    
        Ok(author)
    }
    

    pub fn remove_author(&mut self) {
        match &mut self.authors {
            Some(authors) => {
                if authors.is_empty() {
                    println!("{}", "No authors to remove.".yellow());
                    return;
                }
    
                let prompt = "Which author would you like to remove?";
                let author_emails: Vec<&str> = authors.iter().map(|author| author.email.as_str()).collect();
                let selected_email = Dialog::select_str(prompt, &author_emails, None, None, None);
    
                if let Some(index) = authors.iter().position(|author| author.email == selected_email) {
                    authors.remove(index);
    
                    // Check if the removed author is the default author
                    if self.default_author == Some(selected_email.to_string()) {
                        self.default_author = None;
    
                        // If there's at least one author left, set it as the default
                        if !authors.is_empty() {
                            self.default_author = Some(authors[0].email.clone());
                        }
                    }
    
                    println!("Removed author: {}", selected_email);
                } else {
                    println!("Author not found.");
                }
            }
            None => println!("{}", "No authors configured".yellow()),
        }
    }
    

    pub fn set_default_author(&mut self) {
        match &mut self.authors {
            Some(authors) => {
                if authors.is_empty() {
                    println!("No authors to remove.");
                    return;
                }
                let prompt = "Which author would you like to set as default?";
                let author_emails: Vec<&str> = authors.iter().map(|author| author.email.as_str()).collect();
                let selected_email = Dialog::select_str(prompt, &author_emails, None, None, None);
                self.default_author = Some(selected_email);

            }
            None => println!("No authors configured"),
        }
    }

    pub fn set_default_license(&mut self) -> anyhow::Result<String> {
        let license: String = Dialog::input("Set default license", None, None, None);
        self.default_license = Some(license.clone());
        Ok(license)
    }

    pub fn display(&self) {
        match &self.authors {
            Some(authors) => {
                println!("\nCurrent authors:");
                for (i, author) in authors.iter().enumerate() {
                    let default_author = match self.default_author.clone() {
                        Some(author_email) => author_email,
                        None => "".to_string()
                    };
                    let is_default_author = if default_author == author.email {
                        " (Default)"
                    } else { "" };
                    println!("\n{}{}", format!("Author {}", i + 1).blue().bold(), is_default_author.blue());
                    println!("Name: {}", author.name);
                    println!("Email: {}", author.email);
                    println!("Organization: {}", author.organization);
                
                }
            }
            None => {
                println!("{}", "No authors configured".yellow());
            }
        }
        match &self.default_license {
            Some(license) => println!("\n{}{}", "Default License: ".blue().bold(), license),
            None => println!("\n{}", "No license configured".yellow())
        }
        println!("\n");
    }
}