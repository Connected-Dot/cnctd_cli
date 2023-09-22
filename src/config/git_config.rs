use std::{thread, time::Duration};
use cnctd::{cnctd_git::{account::GitAccount, GitProvider}, cnctd_dialogue::Dialog};
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GitConfig {
    pub git_accounts: Option<Vec<GitAccount>>,
    pub default_account: Option<String>,
}

impl GitConfig {
    // #[async_recursion]
    pub async fn add_account(&mut self) -> anyhow::Result<()> {
        let prompt = "Enter your GitHub token\n";
        let token: String = Dialog::input(prompt, None, None, None);
        let git_account = GitAccount::new(GitProvider::GitHub, &token).await?;
        let login = git_account.login.clone();
    
        match &mut self.git_accounts {
            Some(accounts) => {
                if accounts.iter().any(|account| account.login == login) {
                    println!("{}", "This account already exists.".yellow());
                    thread::sleep(Duration::from_secs(2));
                } else {
                    accounts.push(git_account.clone());
                    if accounts.len() == 1 {
                        self.default_account = Some(login);
                    }
                }
            },
            None => {
                self.git_accounts = Some(vec![git_account]);
                self.default_account = Some(login);
            }
        }
        Ok(())
    }

    pub fn remove_account(&mut self) -> anyhow::Result<()> {
        match &mut self.git_accounts {
            Some(git_accounts) => {
                if git_accounts.is_empty() {
                    println!("{}", "No Git accounts configured".yellow());
                } else {
                    let prompt = "Which account would you like to remove?";
                    let account_names: Vec<&str> = git_accounts.iter().map(|acc| acc.login.as_str()).collect();
                    let selected_login = Dialog::select_str(prompt, &account_names, None, None, None);
    
                    if let Some(index) = git_accounts.iter().position(|acc| acc.login == selected_login) {
                        git_accounts.remove(index);
    
                        // Check if the removed account is the default account
                        if self.default_account == Some(selected_login.clone()) {
                            self.default_account = None;
    
                            // If there's at least one account left, set it as the default
                            if !git_accounts.is_empty() {
                                self.default_account = Some(git_accounts[0].login.clone());
                            }
                        }
                        println!("Successfully removed the selected Git account.");
                    } else {
                        println!("No account selected for removal.");
                    }
                }
            }
            None => {
                println!("{}", "No Git accounts configured".yellow());
            }
        }
        Ok(())
    }
    

    pub fn set_default_account(&mut self) -> anyhow::Result<()> {
        match &mut self.git_accounts {
            Some(git_accounts) => {
                if git_accounts.is_empty() {
                    println!("{}", "No Git accounts configured".yellow());
                    return Ok(());
                }

                let prompt = "Which account would you like to set as default?";
                let account_names: Vec<&str> = git_accounts.iter().map(|acc| acc.login.as_str()).collect();
                let selected_login = Dialog::select_str(prompt, &account_names, None, None, None);

                self.default_account = Some(selected_login);

            }
            None => {
                println!("{}", "No Git accounts configured".yellow());
            }
        }
        Ok(())
    }

    pub fn set_default_url(&mut self) -> anyhow::Result<()> {
        match &mut self.git_accounts {
            Some(git_accounts) => {
                if git_accounts.is_empty() {
                    println!("{}", "No Git accounts configured".yellow());
                    return Ok(());
                }
    
                let prompt = "Which account would you like to edit default URL for?";
                let account_names: Vec<String> = git_accounts.iter().map(|acc| acc.login.clone()).collect();
                let selected_login = Dialog::select_str(prompt, &account_names.iter().map(String::as_str).collect::<Vec<_>>(), None, None, None);
    
                if let Some(selected_account) = git_accounts.iter_mut().find(|acc| acc.login == selected_login) {
                    let mut all_urls: Vec<String> = vec![selected_account.personal_url.clone()];
                    for org_url in &selected_account.org_urls {
                        all_urls.push(org_url.clone());
                    }
                    let prompt = "Choose the default URL";
                    let selected_default = Dialog::select_str(prompt, &all_urls.iter().map(String::as_str).collect::<Vec<_>>(), None, None, None);
                    selected_account.default_url = selected_default;
                } else {
                    println!("Account not found.");
                }
            }
            None => {
                println!("{}", "No Git accounts configured".yellow());
            }
        }
        Ok(())
    }

    pub fn display_accounts(&self) {
        match &self.git_accounts {
            Some(accounts) => {
                println!("\nCurrent accounts:");
                for (i, account) in accounts.iter().enumerate() {
                    let default_account = match self.default_account.clone() {
                        Some(login) => login,
                        None => "".to_string()
                    };
                    let is_default_account = if default_account == account.login {
                        " (Default)"
                    } else { "" };
                    println!("\n{}{}", format!("Account {}", i + 1).blue().bold(), is_default_account.blue());
                    println!("Login: {}", account.login);
                    println!("Token: {}", account.token);
                    let default_url = &account.default_url;
                    fn is_default(url: &str, default_url: &str) -> String {
                        if url == default_url {
                            " (Default)".into()
                        } else { "".into() }
                    }
                    println!("Personal URL: {}{}", account.personal_url, is_default(&account.personal_url, default_url).blue());
                    if account.org_urls.len() > 0 { 
                        for url in &account.org_urls {
                            println!("Org URL: {}{}", url, is_default(url, &default_url).blue());
                        }
                    }
                }
            }
            None => println!("{}", "No Git accounts configured".yellow())
        }
        
        println!("\n");
    }
}