/* 
Config file is specified to derive:
    - function returning configuration specification for actuators GUI builder

    AppConfig derives:
        - contructor: list of modules (ButtonModules) containing lists of buttons (Actuator) with its coresponding name, code and state

*/
use std::{fs::read_to_string, path::Path, sync::Arc};
use egui::mutex::Mutex;
use serde::Deserialize;
use serde::Deserializer;


// Configuration read for page setup and build

// Top actuators config struct
#[derive(Deserialize)]
struct ActuatorsBuff {
    #[serde(rename = "sections")]
    pub actuators: Vec<ActuatorsConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Actuator {
    pub name: String,
    pub code: u8,
    #[serde(skip)] // skip the is_active filed; it is to be used as internal progame marker fo actuator state
    pub is_active: bool,
}

#[derive(Deserialize, Debug)]
pub struct ActuatorsConfig {
    pub module: String,
    pub buttons: Vec<Actuator>,
}

// Top application config structure
#[derive(Deserialize)]
pub struct AppConfig {
    #[serde(rename = "sections")]
    pub actuators: Vec<ActuatorsConfig>,
    pub procedures: Vec<ProceduresConfig>
}


// Procedure structures

// Top procedure structure
#[derive(Deserialize, Clone, Debug)]
pub struct ProceduresConfig {
    pub name: String,
    pub config: RunConfig,
    pub phases: Vec<Phase>
}

#[derive(Deserialize, Clone, Debug)]
pub struct RunConfig {
    #[serde(rename = "loop")]
    pub loop_enabled: bool,
    pub iterations: Option<usize>
}

#[derive(Deserialize, Clone, Debug)]
pub struct PhaseActuator {
    pub name: String,
    #[serde(deserialize_with = "AppConfig::bool_from_int_or_bool")]
    pub state: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Phase {
    pub name: String,
    pub duration_sec: u64,
    pub actuators: Vec<PhaseActuator>
}


// Error handling enum
#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(serde_yaml::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "Error while reading file: {}", e),
            ConfigError::Parse(e) => write!(f, "Error while parsing a config file {}", e),
        }
    }
}



impl AppConfig {
    pub fn new() -> Self {
        Self { actuators: Vec::new(), procedures: Vec::new() }
    }

    pub fn load_act_config(&mut self, path: &Path) -> Result<(), ConfigError> {
        let contents: String = read_to_string(path).map_err(ConfigError::Io)?;
        let parsed: ActuatorsBuff = serde_yaml::from_str(&contents).map_err(ConfigError::Parse)?;
        self.actuators = parsed.actuators;
        Ok(())
    }

    pub fn load_procedure_config(&mut self, dir_path: &Path) -> Result<(), ConfigError> {
        if let Ok(entries) = std::fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    let contents = read_to_string(&path).map_err(ConfigError::Io)?;
                    let proc: ProceduresConfig = serde_yaml::from_str(&contents).map_err(ConfigError::Parse)?;
                    
                    self.procedures.push(proc);
                }
            }
        }
        Ok(())
    }

    // helper function for ActuatorsRegister
    pub fn actuators_flat(&self) -> Vec<Actuator> {
        self.actuators
            .iter()
            .flat_map(|module| {
                module.buttons.iter().map(|btn| Actuator {
                    name: btn.name.clone(),
                    code: btn.code,
                    is_active: false,
                })
            })
            .collect()
    }

    fn bool_from_int_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error> where D: Deserializer<'de>, {
        // Deklarujemy pomocniczy enum, który akceptuje różne typy z YAML
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum IntOrBool {
            Int(i64),
            Bool(bool),
        }

        match IntOrBool::deserialize(deserializer)? {
            IntOrBool::Int(0) => Ok(false),
            IntOrBool::Int(1) => Ok(true),
            IntOrBool::Int(i) => Err(serde::de::Error::custom(format!("Oczekiwano 0 lub 1, otrzymano {}", i))),
            IntOrBool::Bool(b) => Ok(b),
        }
    }
}


// Global structure for actuators states managing

pub struct ActuatorsRegister {
    pub items: Arc<Mutex<Vec<Actuator>>>
}

impl ActuatorsRegister {
    pub fn from_config(config: &AppConfig) -> Self {
        Self { items: Arc::new(Mutex::new(config.actuators_flat())) }
    }

    pub fn toggle_state(&self, code: u8) -> bool {
        let mut items = self.items.lock();
        if let Some(actuator) = items.iter_mut().find(|a| a.code == code) {
            actuator.is_active = !actuator.is_active;
            return actuator.is_active;
        }
        false
    }

    pub fn reset_all(&self) {
        let mut items = self.items.lock();
        for item in items.iter_mut() {
            item.is_active = false;
        }
    }
}