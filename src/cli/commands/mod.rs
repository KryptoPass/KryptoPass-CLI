mod create;
mod generate;

pub use create::CreateCommand;
pub use generate::GenerateCommand;

pub use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "create", subcommand)]
    Create(CreateCommand),

    #[command(name = "generate", subcommand)]
    Generate(GenerateCommand),
}

pub trait Command {
    fn execute(&self);
}
