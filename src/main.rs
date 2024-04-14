mod cli;
mod modules;

use cli::*;

fn main() {
    let app = CLI::parse();
    app.bootstrap();
}
