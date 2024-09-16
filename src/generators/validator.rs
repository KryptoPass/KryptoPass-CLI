use super::config::{Config, LanguageSets, LengthRequirement, Requirements, SetRequirement};
use super::utils::{parse_pattern, PatternElement};
use std::collections::HashMap;
use std::error::Error;

// Validar la compatibilidad de las secciones
pub fn validate_config(config: &Config) -> Result<(), Box<dyn Error>> {
    if let Some(rules) = &config.rules {
        if let Some(pattern_str) = &rules.pattern {
            let pattern =
                parse_pattern(pattern_str).ok_or("El patrón no pudo ser parseado correctamente")?;

            validate_pattern(&pattern, &config.languages)?;
        }
    }

    // Validar la compatibilidad de los requirements con el length
    // validate_requirements_length_compatibility(&config.requirements)?;

    // Aplicar la validación de not_allowed a los conjuntos
    // validate_not_allowed(&config)?;

    Ok(())
}

// Validar el patrón con los idiomas
fn validate_pattern(
    pattern: &[PatternElement],
    languages: &HashMap<String, LanguageSets>,
) -> Result<(), Box<dyn Error>> {
    let mut min_length = 0;
    let mut max_length = 0;

    for element in pattern {
        match element {
            PatternElement::Set { name, min, max, .. } => {
                // Validar que el conjunto exista en los idiomas
                validate_set_existence(name, languages)?;
                // Sumar los valores mínimos y máximos
                min_length += min;
                max_length += max;
            }
            PatternElement::Wildcard => {
                // Manejar el elemento "Wildcard"
            }
        }
    }

    Ok(())
}

// Validar que el conjunto exista en los idiomas
fn validate_set_existence(
    name: &str,
    languages: &HashMap<String, LanguageSets>,
) -> Result<(), Box<dyn Error>> {
    let exists = languages
        .values()
        .any(|language| language.sets.contains_key(name));

    if !exists {
        return Err(format!("El conjunto {} no existe en los idiomas.", name).into());
    }

    Ok(())
}

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
