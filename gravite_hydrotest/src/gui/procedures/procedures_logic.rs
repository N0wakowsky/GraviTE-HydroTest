use std::time::{Duration, Instant};

use crate::config::{ActuatorsRegister, ProceduresConfig};

enum RunnerState {
    Idle,
    Initalizing,
    Running {
        phase_idx: usize,
        iteration: u32,
        phase_start: Instant,
    }
}

pub struct ProcedureRunner {
    active_procedure: Option<ProceduresConfig>,
    state: RunnerState
}

impl ProcedureRunner {
    pub fn new() -> Self {
        Self { active_procedure: None, state: RunnerState::Idle }
    }

    pub fn start(&mut self, procedure: ProceduresConfig) {
        self.active_procedure = Some(procedure);
        self.state = RunnerState::Initalizing;
    }

    pub fn stop(&mut self) {
        self.active_procedure = None;
        self.state = RunnerState::Idle;
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, RunnerState::Initalizing | RunnerState::Running { .. })
    }

    pub fn tick(&mut self, register: &ActuatorsRegister) -> Vec<u8> {
        let mut commands_to_send = Vec::new();

        let procedure = match &self.active_procedure {
            Some(p) => p,
            None => return commands_to_send,
        };

        match self.state {
            RunnerState::Idle => {},
            RunnerState::Initalizing => {
                register.reset_all();

                self.state = RunnerState::Running { phase_idx: 0, iteration: 0, phase_start: Instant::now() - Duration::from_secs(9999) };
            }
            RunnerState::Running { mut phase_idx, mut iteration, phase_start } => {
                let current_phase = &procedure.phases[phase_idx];

                if phase_start.elapsed().as_secs() >= current_phase.duration_sec {
                    phase_idx += 1;

                    if phase_idx >= procedure.phases.len() {
                        phase_idx = 0;
                        iteration += 1;

                        let iter = procedure.config.iterations.unwrap_or(1) as u32;
                        if !procedure.config.loop_enabled || iteration >= iter {
                            self.stop();
                            return commands_to_send;
                        }
                    }

                    let next_phase = &procedure.phases[phase_idx];
                    let items = register.items.lock();

                    for target_act in &next_phase.actuators {
                        if let Some(reg_act) = items.iter().find(|a| a.name == target_act.name) {
                            if reg_act.is_active != target_act.state {
                                commands_to_send.push(reg_act.code)
                            }
                        }
                    }

                    self.state = RunnerState::Running { phase_idx, iteration, phase_start: Instant::now() }
                }
            }
        }

        commands_to_send
    }
}