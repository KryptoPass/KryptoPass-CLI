use super::{
    config::{Config, LanguageSets, LengthRequirement, Requirements, Rules, SetRequirement},
    utils::{parse_pattern, PatternElement},
};
use std::collections::HashMap;
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

    #[error("El patrón tiene un valor mínimo mayor al máximo.")]
    InvalidLengthBounds,
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
        let pattern = parse_pattern(pattern_str).ok_or(ValidationError::InvalidPattern)?;
        validate_pattern_length(&pattern, &config.languages, &config.requirements.length)?;
    }

    Ok(())
}

fn validate_pattern_length(
    pattern: &[PatternElement],
    languages: &HashMap<String, LanguageSets>,
    length: &LengthRequirement,
) -> ValResult {
    let (min_length, max_length) = validate_pattern(pattern, languages)?;

    match length {
        LengthRequirement::Range { min, max } => {
            if min_length < *min || max_length > *max {
                return Err(ValidationError::LengthMismatch {
                    min: *min,
                    max: *max,
                    min_len: min_length,
                    max_len: max_length,
                });
            }
        }
        LengthRequirement::Exact(len) => {
            if min_length != *len || max_length != *len {
                return Err(ValidationError::LengthMismatch {
                    min: *len,
                    max: *len,
                    min_len: min_length,
                    max_len: max_length,
                });
            }
        }
    }

    Ok(())
}

// Validar el patrón con los idiomas
fn validate_pattern(
    pattern: &[PatternElement], // Usamos slice en vez de Vec
    languages: &HashMap<String, LanguageSets>,
) -> Result<(usize, usize), ValidationError> {
    let mut min_length = 0;
    let mut max_length = 0;

    for element in pattern {
        match element {
            PatternElement::Set { name, min, max, .. } => {
                // Validar que el conjunto exista en los idiomas
                if !languages.values().any(|lang| lang.sets.contains_key(name)) {
                    return Err(ValidationError::SetNotFound(name.clone()));
                }

                // Sumar los valores mínimos y máximos
                min_length += min;
                max_length += max;
            }
            PatternElement::Wildcard => {
                // Manejar el elemento "Wildcard"
            }
        }

        // Validar que los valores mínimos y máximos sean consistentes
        if min_length > max_length {
            return Err(ValidationError::InvalidLengthBounds);
        }
    }

    Ok((min_length, max_length))
}
