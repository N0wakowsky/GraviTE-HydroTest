/* 
Config file is specified to derive:
    - function returning configuration specification for actuators GUI builder

    AppConfig derives:
        - contructor: list of modules (ButtonModules) containing lists of buttons (Actuator) with its coresponding name, code and state

*/
use std::{fs::read_to_string, sync::{Arc, mpsc::Sender}};
use egui::mutex::Mutex;

use serde::Deserialize;

use crate::gui::pages::serial_control::SerialCommand;

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

#[derive(Deserialize, Debug, Clone)]
pub struct ActuatorsConfig {
    pub module: String,
    pub buttons: Vec<Actuator>,
}

// Top application config structure
#[derive(Deserialize, Debug, Clone)]
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
#[allow(dead_code)]
pub struct PhaseActuator {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Phase {
    pub name: String,
    pub duration_sec: u64,
    pub actuators: Vec<PhaseActuator>
}



impl AppConfig {
    pub fn new() -> Self {
        Self { actuators: Vec::new(), procedures: Vec::new() }
    }

    pub fn load_act_config<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let contents: String = read_to_string(path)?;
        let parsed: ActuatorsBuff = serde_yaml::from_str(&contents)?;
        self.actuators = parsed.actuators;
        Ok(())
    }

    pub fn load_procedure_config<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    let contents = read_to_string(&path)?;
                    let proc: ProceduresConfig = serde_yaml::from_str(&contents)?;
                    
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
}


// Global structure for actuators states managing
#[derive(Clone)]
pub struct ActuatorsRegister {
    pub items: Arc<Mutex<Vec<Actuator>>>
}

impl ActuatorsRegister {
    pub fn from_config(config: &AppConfig) -> Self {
        Self { items: Arc::new(Mutex::new(config.actuators_flat())) }
    }

    pub fn toggle_state(&self, code: u8) {
        let mut items = self.items.lock();
        if let Some(actuator) = items.iter_mut().find(|a| a.code == code) {
            actuator.is_active = !actuator.is_active;
        }
    }

    pub fn reset_all(&self, tx_serial: &Sender<SerialCommand>) {
        let mut items = self.items.lock();
        for item in items.iter_mut() {
            if item.is_active {
                let _ = tx_serial.send(SerialCommand::TogglePeripheral(item.code));
            }
        }
    }

    pub fn set_active_by_name(&self, name: &String, tx_serial: &Sender<SerialCommand>) {
        let code = {
            let items = self.items.lock();
            items.iter().find(|a| a.name == *name).map(|a| a.code)
        };

        if let Some(code) = code {
            let _ = tx_serial.send(SerialCommand::TogglePeripheral(code));
        }
    }
}