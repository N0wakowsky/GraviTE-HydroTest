use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub sections: Vec<SectionConfig>
}

#[derive(Deserialize)]
pub struct SectionConfig {
    pub name: String,
    pub buttons: Vec<ButtonConfig>
}

#[derive(Deserialize)]
pub struct ButtonConfig {
    pub name: String,
    pub code: u8,
}

pub enum ConfigError {
    Io(std::io::Error),
    Parse(serde_yaml::Error),
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let contents = std::fs::read_to_string(path).map_err(ConfigError::Io)?;
        serde_yaml::from_str(&contents).map_err(ConfigError::Parse)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            sections: vec![
                SectionConfig {
                    name: "Zawory".into(),
                    buttons: (1..=8).map(|i| ButtonConfig {
                        name: format!("Zawór {}", i),
                        code: i as u8,
                    }).collect(),
                },
                SectionConfig {
                    name: "Piezopompy".into(),
                    buttons: (1..=4).map(|i| ButtonConfig {
                        name: format!("Piezopompa {}", i),
                        code: 0x10 + i as u8,
                    }).collect(),
                },
                SectionConfig {
                    name: "Pompy perystaltyczne".into(),
                    buttons: (1..=8).map(|i| ButtonConfig {
                        name: format!("Pompa {}", i),
                        code: 0x20 + i as u8,
                    }).collect(),
                },
            ]
        }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "Error while reading file: {}", e),
            ConfigError::Parse(e) => write!(f, "Error while parsing a config file {}", e),
        }
    }
}