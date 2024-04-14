use crate::cli::commands::Command;
pub use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum CreateCommand {
    #[command(name = "rule")]
    Rule(CreateRule),
}

#[derive(Debug, Args)]
pub struct CreateRule {
    #[arg(long)]
    name: String,

    #[arg(long)]
    description: String,
}

impl Command for CreateRule {
    fn execute(&self) {
        println!(
            "Ejecutando comando CreateRule con nombre: {} y descripción: {}",
            self.name, self.description
        );
    }
}
