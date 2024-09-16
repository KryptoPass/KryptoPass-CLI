
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

















// // Validar que el pattern sea compatible con los requirements y los sets de lenguajes
// fn validate_pattern_requirements_compatibility(
//     requirements: &Requirements,
//     languages: &HashMap<String, HashMap<String, Vec<String>>>,
//     pattern: &[PatternElement],
// ) -> Result<(), Box<dyn Error>> {
//     let mut min_length = 0;
//     let mut max_length = 0;

//     for element in pattern {
//         match element {
//             PatternElement::Set {
//                 name,
//                 min,
//                 max,
//                 negate,
//             } => {
//                 // Verificar si el conjunto existe
//                 let exists = languages.values().any(|sets| sets.contains_key(name));
//                 if !exists {
//                     return Err(format!("El conjunto {} no existe en los idiomas.", name).into());
//                 }

//                 // Sumar las cantidades mínimas y máximas
//                 min_length += min;
//                 max_length += max;
//             }
//             PatternElement::Any => {
//                 // Aquí podrías definir cómo manejar "Any", por ahora solo suma 1
//                 min_length += 1;
//                 max_length += 1;
//             }
//             PatternElement::Wildcard => {
//                 // Aquí podrías definir cómo manejar "Wildcard", por ahora solo suma 1
//                 min_length += 1;
//                 max_length += 1;
//             }
//         }
//     }

//     // Validar que las longitudes del pattern sean compatibles con los requirements
//     match &requirements.length {
//         LengthRequirement::Range { min, max } => {
//             if min_length < *min || max_length > *max {
//                 return Err(format!(
//                     "El patrón no es compatible con los límites de longitud. Mínimo permitido: {}, máximo permitido: {}. Longitud del patrón: {}-{}.",
//                     min, max, min_length, max_length
//                 ).into());
//             }
//         }
//         LengthRequirement::Exact(len) => {
//             if min_length != *len || max_length != *len {
//                 return Err(format!(
//                     "El patrón no es compatible con la longitud exacta requerida: {}.",
//                     len
//                 )
//                 .into());
//             }
//         }
//     }

//     Ok(())
// }

// Validar que los requirements sean compatibles con el length
fn validate_requirements_length_compatibility(
    requirements: &Requirements,
) -> Result<(), Box<dyn Error>> {
    let mut min_sum = 0;
    let mut max_sum = 0;

    // Sumar los mínimos y máximos de cada conjunto en los requirements
    for (_, requirement) in &requirements.sets {
        match requirement {
            SetRequirement::Range { min, max } => {
                min_sum += min;
                max_sum += max.unwrap_or(*min);
            }
            SetRequirement::Exact(val) => {
                min_sum += val;
                max_sum += val;
            }
        }
    }

    // Validar que la suma mínima y máxima de los conjuntos no sea inconsistente con length
    match &requirements.length {
        LengthRequirement::Range { min, max } => {
            if min_sum > *max || max_sum < *min {
                return Err(format!(
                    "La suma de los requisitos no es compatible con la longitud. Suma mínima: {}, suma máxima: {}. Longitud permitida: {}-{}.",
                    min_sum, max_sum, min, max
                ).into());
            }
        }
        LengthRequirement::Exact(len) => {
            if min_sum != *len || max_sum != *len {
                return Err(format!(
                    "Los requisitos no son compatibles con la longitud exacta requerida: {}.",
                    len
                )
                .into());
            }
        }
    }

    Ok(())
}

// // Aplicar las reglas de not_allowed a los conjuntos de caracteres
// fn validate_not_allowed(config: &Config) -> Result<(), Box<dyn Error>> {
//     if let Some(not_allowed) = &config.not_allowed {
//         for (_, sets) in &config.languages {
//             for (set_name, chars) in sets {
//                 let filtered_chars: Vec<char> = chars
//                     .iter()
//                     .filter(|ch| !not_allowed.exclude.contains(&ch.to_string()))
//                     .cloned()
//                     .collect();

//                 if filtered_chars.is_empty() {
//                     return Err(format!(
//                         "El conjunto {} quedó vacío después de aplicar not_allowed.",
//                         set_name
//                     )
//                     .into());
//                 }
//             }
//         }
//     }

//     Ok(())
// }



use super::utils::{parse_pattern, PatternElement};
use std::collections::HashMap;
use std::error::Error;

type ValResult = Result<(), Box<dyn Error>>;

// Validar la compatibilidad de las secciones
pub fn validate_config(config: &Config) -> ValResult {
    if let Some(rules) = &config.rules {
        validate_rules(rules, config)?;
    }

    Ok(())
}

fn validate_rules(rules: &Rules, config: &Config) -> ValResult {
    if let Some(pattern_str) = &rules.pattern {
        let pattern =
            parse_pattern(pattern_str).ok_or("El patrón no pudo ser parseado correctamente")?;

        validate_pattern_length(&pattern, &config.languages, &config.requirements.length)?;
    }

    Ok(())
}

fn validate_pattern_length(
    pattern: &Vec<PatternElement>,
    languages: &HashMap<String, LanguageSets>,
    length: &LengthRequirement,
) -> ValResult {
    let (min_length, max_length) = validate_pattern(pattern, languages)?;

    match length {
        LengthRequirement::Range { min, max } => {
            if min_length < *min || max_length > *max {
                return Err(format!(
                    "El patrón no es compatible con los límites de longitud. Mínimo permitido: {}, máximo permitido: {}. Longitud del patrón: {}-{}.",
                    min, max, min_length, max_length
                ).into());
            }
        }
        LengthRequirement::Exact(len) => {
            if min_length != *len || max_length != *len {
                return Err(format!(
                    "El patrón no es compatible con la longitud exacta requerida: {}.",
                    len
                )
                .into());
            }
        }
    }

    Ok(())
}

// Validar el patrón con los idiomas
fn validate_pattern(
    pattern: &[PatternElement],
    languages: &HashMap<String, LanguageSets>,
) -> Result<(usize, usize), Box<dyn Error>> {
    let mut min_length = 0;
    let mut max_length = 0;

    for element in pattern {
        match element {
            PatternElement::Set { name, min, max, .. } => {
                // Validar que el conjunto exista en los idiomas
                let exists = languages
                    .values()
                    .any(|language| language.sets.contains_key(name));

                if !exists {
                    return Err(format!("El conjunto {} no existe en los idiomas.", name).into());
                }

                // Sumar los valores mínimos y máximos
                min_length += min;
                max_length += max;
            }
            PatternElement::Wildcard => {
                // Manejar el elemento "Wildcard"
                // Asegurar que el wildcard no reduzca la longitud mínima, pero puede expandir hasta el máximo permitido
                max_length = usize::MAX;
            }
        }

        // Validar que los valores mínimos y máximos sean consistentes
        if min_length > max_length {
            return Err("El patrón tiene un valor mínimo mayor al máximo.".into());
        }
    }

    Ok((min_length, max_length))
}
