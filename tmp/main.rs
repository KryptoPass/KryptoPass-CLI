use rand::seq::SliceRandom; // Necesario para shuffle
use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    properties: Properties,
    requirements: Requirements,
    allowed: Option<Allowed>,
    not_allowed: Option<NotAllowed>,
    #[serde(rename = "rules")]
    rules: Option<Rules>,
    #[serde(flatten)]
    languages: HashMap<String, LanguageSets>,
}

#[derive(Debug, Deserialize)]
struct Properties {
    version: String,
    lang: Vec<String>,
    name: String,
    #[serde(rename = "type")]
    generation_type: String,
}

#[derive(Debug, Deserialize)]
struct Requirements {
    length: LengthRequirement,
    #[serde(flatten)]
    sets: HashMap<String, SetRequirement>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum LengthRequirement {
    Range { min: usize, max: usize },
    Exact(usize),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SetRequirement {
    Range { min: usize, max: Option<usize> },
    Exact(usize),
}

#[derive(Debug, Deserialize)]
struct Allowed {
    include: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct NotAllowed {
    exclude: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)] // Implementa Clone para Rules
struct Rules {
    #[serde(rename = "max-consecutive")]
    max_consecutive: Option<usize>,
    #[serde(rename = "min-entropy-bits")]
    min_entropy_bits: Option<f64>,
    pattern: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LanguageSets {
    #[serde(flatten)]
    sets: HashMap<String, Vec<String>>,
}

fn main() {
    // Lee el archivo de configuración
    let config_content =
        fs::read_to_string("C:\\Users\\maizo\\Videos\\KryptoPass-CLI\\config.toml")
            .expect("No se pudo leer el archivo de configuración");
    let config: Config =
        toml::from_str(&config_content).expect("Error al parsear el archivo de configuración");

    // Genera la contraseña
    let password = generate_password(&config).expect("Error al generar la contraseña");
    println!("Contraseña generada: {}", password);
}

fn generate_password(config: &Config) -> Option<String> {
    // Obtiene el idioma y los conjuntos de caracteres correspondientes
    let lang = &config.properties.lang[0]; // Usaremos el primer idioma de la lista
    let language_sets = config.languages.get(lang)?;

    println!("Sets: {language_sets}");

    // Combina los conjuntos de caracteres según los requisitos
    let mut char_sets = HashMap::new();
    for (set_name, chars) in &language_sets.sets {
        let expanded_chars = expand_unicode_sequences(chars);
        char_sets.insert(set_name.clone(), expanded_chars);
    }

    // Aplica los caracteres permitidos y excluidos
    let allowed_chars = if let Some(allowed) = &config.allowed {
        expand_unicode_sequences(&allowed.include)
    } else {
        vec![]
    };

    let not_allowed_chars = if let Some(not_allowed) = &config.not_allowed {
        expand_unicode_sequences(&not_allowed.exclude)
    } else {
        vec![]
    };

    // Genera la contraseña respetando los requisitos
    let password = build_password(
        &config.requirements,
        &char_sets,
        &allowed_chars,
        &not_allowed_chars,
        &config.rules,
    )?;
    Some(password)
}

fn expand_unicode_sequences(sequences: &Vec<String>) -> Vec<char> {
    let mut chars = Vec::new();
    for seq in sequences {
        if seq.contains('-') {
            let parts: Vec<&str> = seq.split('-').collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (
                    u32::from_str_radix(&parts[0][2..], 16),
                    u32::from_str_radix(&parts[1][2..], 16),
                ) {
                    for code in start..=end {
                        if let Some(ch) = std::char::from_u32(code) {
                            chars.push(ch);
                        }
                    }
                }
            }
        } else {
            if let Ok(code) = u32::from_str_radix(&seq[2..], 16) {
                if let Some(ch) = std::char::from_u32(code) {
                    chars.push(ch);
                }
            }
        }
    }
    chars
}

fn build_password(
    requirements: &Requirements,
    char_sets: &HashMap<String, Vec<char>>,
    allowed_chars: &Vec<char>,
    not_allowed_chars: &Vec<char>,
    rules: &Option<Rules>,
) -> Option<String> {
    let mut password_chars = Vec::new();
    let mut rng = rand::thread_rng();

    // Determina la longitud de la contraseña
    let password_length = match &requirements.length {
        LengthRequirement::Range { min, max } => {
            let max = *max;
            if max < *min {
                return None;
            }
            rng.gen_range(*min..=max)
        }
        LengthRequirement::Exact(len) => *len,
    };

    // Genera caracteres según los requisitos de cada conjunto
    for (set_name, requirement) in &requirements.sets {
        let set_chars = char_sets.get(set_name)?;
        let count = match requirement {
            SetRequirement::Range { min, max } => {
                let max = max.unwrap_or(set_chars.len());
                if max < *min {
                    return None;
                }
                rng.gen_range(*min..=max)
            }
            SetRequirement::Exact(val) => *val,
        };
        for _ in 0..count {
            let idx = rng.gen_range(0..set_chars.len());
            password_chars.push(set_chars[idx]);
        }
    }

    // Completa la contraseña hasta alcanzar la longitud requerida
    while password_chars.len() < password_length {
        // Combina todos los conjuntos disponibles
        let all_chars: Vec<char> = char_sets.values().flatten().cloned().collect();
        let idx = rng.gen_range(0..all_chars.len());
        password_chars.push(all_chars[idx]);
    }

    // Aplica los caracteres permitidos y excluidos
    if !allowed_chars.is_empty() {
        password_chars.extend(allowed_chars.iter());
    }

    password_chars.retain(|ch| !not_allowed_chars.contains(ch));

    // Aplica las reglas adicionales (max-consecutive, pattern, etc.)
    if let Some(rules) = rules {
        // Implementación de max-consecutive
        if let Some(max_consecutive) = rules.max_consecutive {
            if has_max_consecutive(&password_chars, max_consecutive) {
                // Regenera la contraseña si no cumple con la regla
                return build_password(
                    requirements,
                    char_sets,
                    allowed_chars,
                    not_allowed_chars,
                    &Some(rules.clone()),
                );
            }
        }
        // Implementación de min-entropy-bits y pattern puede agregarse aquí
    }

    // Mezcla los caracteres y construye la contraseña
    password_chars.shuffle(&mut rand::thread_rng());
    Some(password_chars.iter().collect())
}

fn has_max_consecutive(chars: &Vec<char>, max_consecutive: usize) -> bool {
    let mut count = 1;
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
            if count > max_consecutive {
                return true;
            }
        } else {
            count = 1;
        }
    }
    false
}
