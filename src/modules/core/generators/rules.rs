use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Read};

use serde::{self, Deserialize, Serialize};
use toml::map::Map;
use toml::Value as TOMLParser;

#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    pub name: String,
    pub description: String,
    pub lang: String,
    pub version: String,
    pub unique_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Charset {
    pub chars: Vec<char>,
    pub name: String,
    pub code: String,
}

#[derive(Debug)]
pub struct MinMax {
    pub min: u8,
    pub max: u8,
}

#[derive(Debug)]
pub enum Requirement {
    MinMax(MinMax),
    Active(bool),
    Amount(u8),
}

pub struct Rules {
    pub properties: Properties,
    pub charset: HashMap<String, Charset>,
    pub requirements: HashMap<String, Requirement>,
}

impl Rules {
    pub fn from_file(file_path: &str) -> Result<Self, ErrorKind> {
        let mut contents = String::new();

        let mut file = File::open(file_path).unwrap();
        file.read_to_string(&mut contents).unwrap();

        let toml = contents.parse::<TOMLParser>().unwrap();

        let properties_table = toml.get("properties").unwrap();
        let properties_table = properties_table.as_table().unwrap();
        let properties = Properties::deserialize(properties_table.clone()).unwrap();

        let charset_table = toml.get("charset").unwrap();
        let charset_table = charset_table.as_table().unwrap();
        let charset = Self::get_charset(charset_table);

        let requirements_table = toml.get("requirements").unwrap();
        let requirements_table = requirements_table.as_table().unwrap();
        let mut requirements: HashMap<String, Requirement> = HashMap::new();

        for (k, v) in requirements_table {
            if v.is_integer() {
                let value = v.as_integer().unwrap();
                let value = Self::saturate_to_u8(value);

                requirements.insert(k.clone(), Requirement::Amount(value));
                continue;
            } else if v.is_table() {
                let min = v.get("min").unwrap().as_integer().unwrap();
                let min = Self::saturate_to_u8(min);

                let max = v.get("max").unwrap().as_integer().unwrap();
                let max = Self::saturate_to_u8(max);

                requirements.insert(k.clone(), Requirement::MinMax(MinMax { min, max }));
                continue;
            } else if v.is_bool() {
                requirements.insert(k.clone(), Requirement::Active(v.as_bool().unwrap()));
                continue;
            }

            panic!("WTF Bro!?");
        }

        println!("{:#?}", requirements);

        Ok(Rules {
            properties,
            charset,
            requirements,
        })
    }

    pub fn default() -> Self {
        let properties = Properties {
            name: String::from("Default"),
            description: String::from("Default"),
            lang: String::from("en"),
            version: String::from("Default"),
            unique_id: String::from("Default"),
        };

        let mut charset = HashMap::new();
        charset.insert(
            String::from("english"),
            Charset {
                name: String::from("Inglés (EE.UU.)"),
                chars: (32..127).map(|i| i as u8 as char).collect(),
                code: String::from("english"),
            },
        );

        let mut requirements = HashMap::new();
        requirements.insert(String::from("length"), Requirement::Amount(16));
        requirements.insert(String::from("english"), Requirement::Active(true));

        Rules {
            properties,
            charset,
            requirements,
        }
    }

    pub fn saturate_to_u8(v: i64) -> u8 {
        if v > (u8::MAX as i64) {
            u8::MAX
        } else if v < 1 {
            1
        } else {
            v as u8
        }
    }

    fn get_charset(charset_table: &Map<String, TOMLParser>) -> HashMap<String, Charset> {
        let mut charsets: HashMap<String, Charset> = HashMap::new();

        for (k, v) in charset_table {
            if k == "name" {
                continue;
            }

            let name = v.get("name").unwrap().to_string();
            let chars = v.get("values").unwrap().as_array().unwrap();
            let chars = Self::normalize_charset(chars);

            charsets.insert(
                k.clone(),
                Charset {
                    name,
                    chars,
                    code: k.clone(),
                },
            );
        }

        charsets
    }

    fn normalize_charset(charset: &Vec<TOMLParser>) -> Vec<char> {
        let mut result = Vec::new();

        for value in charset {
            match value {
                TOMLParser::Table(table) => {
                    let start = table.get("start").unwrap().as_integer().unwrap() as u32;
                    let end = table.get("end").unwrap().as_integer().unwrap() as u32;
                    for code in start..=end {
                        if let Some(character) = std::char::from_u32(code) {
                            result.push(character);
                        }
                    }
                }
                TOMLParser::Integer(code) => {
                    if let Some(character) = std::char::from_u32(*code as u32) {
                        result.push(character);
                    }
                }
                _ => (),
            }
        }

        result
    }
}
