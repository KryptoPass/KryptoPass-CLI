mod generators;

use generators::PasswordGenerator;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let password_generator = PasswordGenerator::from_file("config.toml")?;

    let password = password_generator.generate()?;

    println!("{}", password);

    Ok(())
}
