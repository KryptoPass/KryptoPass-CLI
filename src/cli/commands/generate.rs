pub use clap::{Args, Subcommand};

use crate::cli::commands::Command;
use crate::modules::core::generators::password::{PasswordEngine, Rules};

#[derive(Debug, Subcommand)]
pub enum GenerateCommand {
    #[command(name = "password")]
    Password(PasswordCommand),
}

#[derive(Debug, Args)]
pub struct PasswordCommand {
    
}

impl Command for PasswordCommand {
    fn execute(&self) {}
}

// let rule = match &self.rule {
//     Some(rule_path) => {
//         let rule = Rules::from_file(rule_path).unwrap();
//         PasswordEngine::from_rules(rule)
//     }
//     None => {
//         // Default rule if no rule file is provided-
//         let rule = Rules::default();
//         PasswordEngine::from_rules(rule)
//     }
// };

// println!("{}", rule.generate().unwrap());
