mod commands;
mod constants;

pub use clap::Parser;

use commands::*;
use constants::KRYPTOPASS_VERSION;

#[derive(Debug, Parser)]
#[command(
    name = "KryptoPass",
    bin_name = "kryptopass",
    version = KRYPTOPASS_VERSION,
    about = "KryptoPass: Password Manager!",
    long_about = "KryptoPass: Your digital fortress of secrets guarded by cryptographic wizardry! 🌟🔐",
    author = "Gabriel Maizo <maizogabriel@outlook.com>"
)]
pub struct CLI {
    #[command(subcommand)]
    commands: Commands,
}

impl CLI {
    pub fn bootstrap(&self) {
        match &self.commands {
            Commands::Create(cmd) => match cmd {
                CreateCommand::Rule(subcmd) => subcmd.execute(),
            },
            Commands::Generate(cmd) => match cmd {
                GenerateCommand::Password(subcmd) => subcmd.execute(),
            },
        }
    }
}
