use std::fs::File;
use std::io::{Error, Read};
use toml::Value;

#[derive(Debug)]
pub enum MinMax {
    Number(u32),
    Range { min: i32, max: i32 },
}

pub struct PasswordRules {
    // Properties
    name: String,
    lang: Vec<String>,
    r#type: String,

    // Requirements
    length: MinMax,
    uppercase: MinMax,
    lowercase: MinMax,
    digits: MinMax,
    symbols: MinMax,

    // Rules
    max_consecutive: u32,
}

impl PasswordRules {
    pub fn from_file(file_path: String) -> Result<(), Error> {
        let mut file = File::open(file_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Process the contents as needed
        let toml = contents.parse::<Value>();

        match toml {
            Ok(content) => {
                let properties = content.get("properties").unwrap();

                println!("{:#?}", properties.as_table().unwrap())
            }
            Err(err) => {
                println!("Error {}", err)
            }
        }

        Ok(())
    }
}

// pub fn new() -> Self {
//     PasswordRules {
//         name: String::from("General Purpose Rules"),
//         lang: vec!["es", "en"],
//         length: Length::Range { min: 6, max: 128 },
//     }
// }

// pub fn from_file(file: String) -> Self {
//     PasswordRules {
//         name: String::from("General Purpose Rules"),
//         lang: vec!["es", "en"],
//         length: Length::Number(8),
//     }
// }
