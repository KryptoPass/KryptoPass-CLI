use super::rules::{MinMax, Requirement, Rules};
use crate::modules::core::crypto::random::Random;

pub struct PasswordGenerator {
    rules: Rules,
}

const DEFAULT_LENGTH: u8 = 14;

impl PasswordGenerator {
    pub fn new() -> Self {
        PasswordGenerator {
            rules: Rules::default(),
        }
    }

    pub fn from_rules(rules: Rules) -> Self {
        PasswordGenerator { rules }
    }

    pub fn generate(&self) -> Result<String, &'static str> {
        let length = self.get_length();

        if !self.check_requirements(length) {
            return Err("Invalid Input");
        }

        let mut password = Vec::new();
        let mut active_chars = Vec::new();

        for (key, requirement) in &self.rules.requirements {
            if key == "length" {
                continue;
            }

            let charset = self.rules.charset.get(key).unwrap();

            match requirement {
                Requirement::MinMax(min_max) => {
                    let MinMax { min, max } = min_max;

                    let amount = Random::rand_range((*min).into(), (*max).into()).unwrap() as u8;

                    for _ in 0..amount {
                        let r = Random::choice(&charset.chars).unwrap();
                        password.push(r.to_string());
                    }
                }
                Requirement::Amount(amount) => {
                    for _ in 0..*amount {
                        let r = Random::choice(&charset.chars).unwrap();
                        password.push(r.to_string());
                    }
                }
                Requirement::Active(is_active) => {
                    if *is_active {
                        for _ in 0..length {
                            let r = Random::choice(&charset.chars).unwrap();
                            active_chars.push(r.to_string());
                        }
                    }
                }
            }
        }

        // Mezcla los caracteres activos y selecciona los que faltan
        Random::shuffle(&mut active_chars);

        while password.len() < length as usize {
            password.push(active_chars.pop().unwrap());
        }

        // Mezcla los caracteres de la contraseña para evitar patrones
        Random::shuffle(&mut password);

        Ok(password.join(""))
    }

    fn check_requirements(&self, length: u8) -> bool {
        let mut count = 0;

        for (key, requirement) in &self.rules.requirements {
            if key == "length" {
                continue;
            }

            match requirement {
                Requirement::MinMax(min_max) => {
                    let MinMax { min, .. } = min_max;
                    count += *min;
                }
                Requirement::Amount(amount) => {
                    count += *amount;
                }
                Requirement::Active(active) => {
                    if *active {
                        if count + 1 > length {
                            count += 1;
                        } else {
                            count = length;
                            break;
                        }
                    }
                }
            }
        }

        if count > length {
            return false;
        }

        true
    }

    fn get_length(&self) -> u8 {
        match self.rules.requirements.get("length").unwrap() {
            Requirement::MinMax(min_max) => {
                let MinMax { min, max } = min_max;
                Random::rand_range((*min).into(), (*max).into()).unwrap() as u8
            }
            Requirement::Amount(len) => *len,
            Requirement::Active(_) => DEFAULT_LENGTH,
        }
    }
}
