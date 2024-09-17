use std::collections::HashMap;

use super::{
    config::{Config, LanguageSets, LengthRequirement, Rules, SetRequirement},
    utils::{check_for_wildcard, parse_pattern, set_exists, PatternElement},
};
use thiserror::Error;

type ValResult = Result<(), ValidationError>;

// Define un tipo de error específico para mejorar el control de errores
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("El conjunto {0} no existe en los idiomas.")]
    SetNotFound(String),

    #[error("El patrón no es compatible con los límites de longitud. Mínimo permitido: {min}, máximo permitido: {max}. Longitud del patrón: {min_len}-{max_len}.")]
    LengthMismatch {
        min: usize,
        max: usize,
        min_len: usize,
        max_len: usize,
    },

    #[error("El patrón no pudo ser parseado correctamente.")]
    InvalidPattern,

    #[error("El patrón tiene un valor mínimo mayor al máximo en el conjunto `{0}`.")]
    InvalidLengthBounds(String),
}

// Validar la compatibilidad de las secciones
pub fn validate_config(config: &Config) -> ValResult {
    if let Some(rules) = &config.rules {
        validate_rules(rules, config)?;
    }

    Ok(())
}

fn validate_rules(rules: &Rules, config: &Config) -> ValResult {
    if let Some(pattern_str) = &rules.pattern {
        validate_pattern(pattern_str, config)?;
    }

    Ok(())
}

fn validate_pattern(pattern_str: &str, config: &Config) -> ValResult {
    // Analizar el pattern: Parsear el patrón para extraer los bloques, conjuntos y cantidades.
    let patterns = parse_pattern(pattern_str).ok_or(ValidationError::InvalidPattern)?;
    let languages = &config.languages;
    let requirements = &config.requirements;

    // Calcular la longitud mínima y máxima posible del pattern
    let mut min_length = 0;
    let mut max_length = 0;

    for pattern in &patterns {
        match pattern {
            PatternElement::Set { name, min, max, .. } => {
                // Verificar que los conjuntos utilizados en el pattern existen en las secciones de idiomas y requisitos.
                if let Err(err) = check_set_exists(name, languages, &requirements.sets) {
                    return Err(err);
                }

                // Validar el conjunto de requisitos
                let set_req = &requirements.sets[name];
                validate_set_requirement(set_req, name, *min, *max)?;

                // Sumar las cantidades mínimas y máximas de cada bloque.
                min_length += min;
                max_length += max;
            }
            _ => (),
        }
    }

    // Comparar con `requirements.length``, para verificar que la longitud del pattern está dentro de los límites definidos.
    let contains_wildcard = check_for_wildcard(&patterns);
    let requirements = &config.requirements;

    match requirements.length {
        LengthRequirement::Range { min, max } => {
            if contains_wildcard {
                if max_length > max {
                    return Err(ValidationError::LengthMismatch {
                        min,
                        max,
                        min_len: min_length,
                        max_len: max_length,
                    });
                }
            } else {
                if min_length < min || max_length > max {
                    return Err(ValidationError::LengthMismatch {
                        min,
                        max,
                        min_len: min_length,
                        max_len: max_length,
                    });
                }
            }
        }
        LengthRequirement::Exact(len) => {
            if contains_wildcard {
                if max_length > len {
                    return Err(ValidationError::LengthMismatch {
                        min: len,
                        max: len,
                        min_len: min_length,
                        max_len: max_length,
                    });
                }
            } else {
                if min_length != len || max_length != len {
                    return Err(ValidationError::LengthMismatch {
                        min: len,
                        max: len,
                        min_len: min_length,
                        max_len: max_length,
                    });
                }
            }
        }
    }

    Ok(())
}

fn validate_set_requirement(
    set_req: &SetRequirement,
    name: &str,
    min: usize,
    max: usize,
) -> ValResult {
    match set_req {
        SetRequirement::Range {
            min: set_min,
            max: Some(set_max),
        } => {
            // Verificar que min <= max en los requisitos del conjunto
            if min > max {
                return Err(ValidationError::InvalidLengthBounds(name.to_string()));
            }
            // Verificar que los valores mínimos no son inválidos respecto al requisito del conjunto
            if *set_min > *set_max {
                return Err(ValidationError::InvalidLengthBounds(name.to_string()));
            }
        }
        SetRequirement::Range {
            min: set_min,
            max: None,
        } => {
            // Verificar que min no excede el límite
            if min > *set_min {
                return Err(ValidationError::InvalidLengthBounds(name.to_string()));
            }
        }
        SetRequirement::Exact(set_len) => {
            // Verificar que min y max son iguales al valor exacto del requisito
            if min != *set_len || max != *set_len {
                return Err(ValidationError::LengthMismatch {
                    min: *set_len,
                    max: *set_len,
                    min_len: min,
                    max_len: max,
                });
            }
        }
    }
    Ok(())
}

fn check_set_exists(
    name: &str,
    languages: &HashMap<String, LanguageSets>,
    requirements: &HashMap<String, SetRequirement>,
) -> ValResult {
    if !set_exists(languages, &name) && !set_exists(requirements, &name) {
        return Err(ValidationError::SetNotFound(name.to_string()));
    }
    Ok(())
}
