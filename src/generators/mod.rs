mod config;
mod utils;
mod validator;

use std::collections::HashMap;
use std::error::Error;
use std::fs;

use rand::seq::SliceRandom;
use rand::Rng;

pub use config::Config;
use validator::validate_config;

pub struct PasswordGenerator {
    config: Config,
}

impl PasswordGenerator {
    pub fn from_config(config: Config) -> Self {
        PasswordGenerator { config }
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(PasswordGenerator { config })
    }

    pub fn generate(&self) -> Result<String, Box<dyn Error>> {
        // Validar la configuración antes de proceder
        validate_config(&self.config)?;

        Ok("Hello, world!".to_string())
    }
}

// mod utils;

// pub use config::{Config, LengthRequirement, Requirements, Rules, SetRequirement};
// use utils::{expand_unicode_sequences, has_max_consecutive, parse_pattern, PatternElement};

// pub fn generate_password(config: &Config) -> Option<String> {
//     // Obtiene el idioma y los conjuntos de caracteres correspondientes
//     let lang: &String = &config.properties.lang[0]; // Usaremos el primer idioma de la lista
//     let language_sets = config.languages.get(lang)?;

//     // Combina los conjuntos de caracteres según los requisitos
//     let mut char_sets = HashMap::new();
//     for (set_name, chars) in &language_sets.sets {
//         let expanded_chars = expand_unicode_sequences(chars);
//         char_sets.insert(set_name.clone(), expanded_chars);
//     }

//     // Aplica los caracteres permitidos y excluidos
//     let allowed_chars = if let Some(allowed) = &config.allowed {
//         expand_unicode_sequences(&allowed.include)
//     } else {
//         vec![]
//     };

//     let not_allowed_chars = if let Some(not_allowed) = &config.not_allowed {
//         expand_unicode_sequences(&not_allowed.exclude)
//     } else {
//         vec![]
//     };

//     // Genera la contraseña respetando los requisitos
//     let password = build_password(
//         &config.requirements,
//         &char_sets,
//         &allowed_chars,
//         &not_allowed_chars,
//         &config.rules,
//     )?;
//     Some(password)
// }

// fn build_password(
//     requirements: &Requirements,
//     char_sets: &HashMap<String, Vec<char>>,
//     allowed_chars: &Vec<char>,
//     not_allowed_chars: &Vec<char>,
//     rules: &Option<Rules>,
// ) -> Option<String> {
//     let mut rng = rand::thread_rng();

//     // Aplica los caracteres permitidos y excluidos
//     let mut all_chars: Vec<char> = char_sets.values().flatten().cloned().collect();

//     if !allowed_chars.is_empty() {
//         all_chars.extend(allowed_chars.iter());
//     }

//     all_chars.retain(|ch| !not_allowed_chars.contains(ch));

//     // Verifica que haya caracteres disponibles
//     if all_chars.is_empty() {
//         return None;
//     }

//     // Si hay un patrón definido, lo utilizamos
//     if let Some(rules) = rules {
//         if let Some(pattern_str) = &rules.pattern {
//             let pattern = parse_pattern(pattern_str)?;

//             // Genera la contraseña siguiendo el patrón
//             let mut password_chars = Vec::new();
//             for element in pattern {
//                 match element {
//                     PatternElement::Set {
//                         name,
//                         min,
//                         max,
//                         negate,
//                     } => {
//                         let set_chars = if negate {
//                             all_chars
//                                 .iter()
//                                 .filter(|ch| !char_sets.get(&name).unwrap_or(&vec![]).contains(ch))
//                                 .cloned()
//                                 .collect::<Vec<char>>()
//                         } else {
//                             char_sets.get(&name)?.clone()
//                         };

//                         if set_chars.is_empty() {
//                             return None;
//                         }

//                         let count = if min == max {
//                             min
//                         } else {
//                             rng.gen_range(min..=max)
//                         };

//                         for _ in 0..count {
//                             let ch = set_chars.choose(&mut rng)?;
//                             password_chars.push(*ch);
//                         }
//                     }
//                     PatternElement::Any => {
//                         // Rellenar con caracteres aleatorios hasta completar la longitud requerida
//                         let required_length = match &requirements.length {
//                             LengthRequirement::Range { min, max } => rng.gen_range(*min..=*max),
//                             LengthRequirement::Exact(len) => *len,
//                         };

//                         while password_chars.len() < required_length {
//                             let ch = all_chars.choose(&mut rng)?;
//                             password_chars.push(*ch);
//                         }
//                     }
//                 }
//             }

//             // Aplica las reglas adicionales
//             if let Some(max_consecutive) = rules.max_consecutive {
//                 if has_max_consecutive(&password_chars, max_consecutive) {
//                     // Regenera la contraseña si no cumple con la regla
//                     return build_password(
//                         requirements,
//                         char_sets,
//                         allowed_chars,
//                         not_allowed_chars,
//                         &Some(rules.clone()),
//                     );
//                 }
//             }

//             // Mezcla los caracteres si no se requiere orden específico
//             password_chars.shuffle(&mut rng);

//             return Some(password_chars.iter().collect());
//         }
//     }

//     // Si no hay patrón, generamos la contraseña siguiendo los requisitos
//     let mut password_chars = Vec::new();

//     // Genera caracteres según los requisitos de cada conjunto
//     for (set_name, requirement) in &requirements.sets {
//         let set_chars = char_sets.get(set_name)?;
//         let count = match requirement {
//             SetRequirement::Range { min, max } => {
//                 let max = max.unwrap_or(*min);
//                 if max < *min {
//                     return None;
//                 }
//                 rng.gen_range(*min..=max)
//             }
//             SetRequirement::Exact(val) => *val,
//         };
//         for _ in 0..count {
//             let ch = set_chars.choose(&mut rng)?;
//             password_chars.push(*ch);
//         }
//     }

//     // Completa la contraseña hasta alcanzar la longitud requerida
//     let password_length = match &requirements.length {
//         LengthRequirement::Range { min, max } => {
//             let max = *max;
//             if max < *min {
//                 return None;
//             }
//             rng.gen_range(*min..=max)
//         }
//         LengthRequirement::Exact(len) => *len,
//     };

//     while password_chars.len() < password_length {
//         let ch = all_chars.choose(&mut rng)?;
//         password_chars.push(*ch);
//     }

//     // Aplica las reglas adicionales
//     if let Some(rules) = rules {
//         if let Some(max_consecutive) = rules.max_consecutive {
//             if has_max_consecutive(&password_chars, max_consecutive) {
//                 // Regenera la contraseña si no cumple con la regla
//                 return build_password(
//                     requirements,
//                     char_sets,
//                     allowed_chars,
//                     not_allowed_chars,
//                     &Some(rules.clone()),
//                 );
//             }
//         }
//     }

//     // Mezcla los caracteres
//     password_chars.shuffle(&mut rng);
//     Some(password_chars.iter().collect())
// }
