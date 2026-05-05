use std::{thread, time::Duration};

use egui::{Color32, Vec2};

use std::sync::mpsc;

use crate::{config::AppConfig, gui::{components::{ButtonTrait, PageTrait}, pages::serial_control::SerialCommand}};

use crate::gui::components::PageContext;
use crate::gui::components::AppState;

use crate::config::ProceduresConfig;
use crate::config::ActuatorsRegister;

#[derive(Clone)]
pub enum ProcedureCommand {
    Start(usize), // start command sends a selected procedure index to the procedure thread
    Abort,
}

#[derive(Clone)]
pub enum ProcedureStatus {
    Idle,
    Running {proc_idx: usize, phase_idx: usize},
    Aborted,
}

struct ProcButton {
    label: String,
    color: Color32,
    size: Vec2,
    instruction: ProcedureCommand,
    tx_proc: mpsc::Sender<ProcedureCommand>,
}

impl ProcButton {
    fn new(instruction: ProcedureCommand, tx_proc: mpsc::Sender<ProcedureCommand>) -> Self {
        let (label, color) = match instruction {
            ProcedureCommand::Start(_) => ("Start".to_string(), Color32::from_rgb(0, 180, 0)),
            ProcedureCommand::Abort => ("Abort".to_string(), Color32::from_rgb(200, 0, 0)),
        };

        Self { 
            label, 
            color, 
            size: Vec2 { x: 100.0, y: 40.0 },
            instruction,
            tx_proc
        }
    }
    fn set_instruction(&mut self, instruction: ProcedureCommand) {
        self.instruction = instruction;
    }
}

impl ButtonTrait for ProcButton {
    fn label(&self) -> &str {
        &self.label
    }
    fn color(&self, _ui: &egui::Ui) -> Color32 {
        self.color
    }
    fn size(&self) -> Vec2 {
        self.size
    }
    fn on_click(&mut self) {
        // handle procedure depending on button
        let _ = self.tx_proc.send(self.instruction.clone());
    }
}

pub fn spawn_proc_thread(
    rx_cmd: mpsc::Receiver<ProcedureCommand>,
    tx_stat: mpsc::Sender<ProcedureStatus>,
    tx_ser: mpsc::Sender<SerialCommand>,
    config: AppConfig,
    ctx: egui::Context,
    act_register: ActuatorsRegister
) {
    thread::spawn(move || {
        loop {
            if let Ok(ProcedureCommand::Start(proc_idx)) = rx_cmd.try_recv() {
                let selected_procedure = &config.procedures[proc_idx];
                let config = &selected_procedure.config;
                let phases = &selected_procedure.phases;

                let mut aborted = false;

                let iterations = if config.loop_enabled {
                    config.iterations.unwrap_or(1)
                } else {
                    1
                };

                'proc_loop: for _ in 0..iterations {
                    for (phase_idx, phase) in phases.iter().enumerate() {
                        act_register.reset_all(&tx_ser);

                        let _ = tx_stat.send(ProcedureStatus::Running { proc_idx, phase_idx });
                        ctx.request_repaint();

                        for periph in &phase.actuators {
                            act_register.set_active_by_name(&periph.name, &tx_ser);
                        }

                        let duration = Duration::from_secs(phase.duration_sec);

                        match rx_cmd.recv_timeout(duration) {
                            Ok(ProcedureCommand::Abort) => {
                                aborted = true;
                                break 'proc_loop;
                            }
                            Ok(ProcedureCommand::Start(_)) => {}
                            Err(mpsc::RecvTimeoutError::Timeout) => {}
                            Err(mpsc::RecvTimeoutError::Disconnected) => {return ;}
                        }
                    }
                };
                
                act_register.reset_all(&tx_ser);

                let final_state = if aborted {
                    ProcedureStatus::Aborted
                } else {
                    ProcedureStatus::Idle
                };

                let _ = tx_stat.send(final_state);
                ctx.request_repaint();
            }
        }
    });
}

pub struct ProcPage {
    start_button: ProcButton,
    abort_button: ProcButton,
    procedures: Vec<ProceduresConfig>,
    current_procedure: Option<usize>,
}

impl ProcPage {
    pub fn new(ctx: &PageContext) -> Self {
        Self { 
            start_button: ProcButton::new(ProcedureCommand::Start(0), ctx.tx_proc.clone()), 
            abort_button: ProcButton::new(ProcedureCommand::Abort, ctx.tx_proc.clone()),
            procedures: ctx.config.procedures.clone(),
            current_procedure: None
        }
    }
}


impl PageTrait for ProcPage {
    fn update(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &AppState) {
        ui.heading("Procedure Panel");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Status:");
            let status_text = match state.proc_state {
                ProcedureStatus::Idle => "Idle",
                ProcedureStatus::Running { .. } => "Running",
                ProcedureStatus::Aborted => "Aborted",
            };
            let status_color = match state.proc_state {
                ProcedureStatus::Idle => Color32::GRAY,
                ProcedureStatus::Running { .. } => Color32::GREEN,
                ProcedureStatus::Aborted => Color32::YELLOW,
            };

            ui.colored_label(status_color, status_text)
        });

        ui.add_space(20.0);
        
        ui.group(|ui| {
            ui.label("Select procedure:");
            for (idx, proc) in self.procedures.iter().enumerate() {
                if ui.selectable_label(self.current_procedure == Some(idx), &proc.name).clicked() {
                    self.current_procedure = Some(idx);
                }
            }
        });

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 15.0;

            if let Some(idx) = self.current_procedure {
                self.start_button.set_instruction(ProcedureCommand::Start(idx));
                
                self.start_button.render(ui);
            } else {
                ui.add_enabled(false, egui::Button::new("Start").min_size(Vec2::new(100.0, 40.0)));
            }

            self.abort_button.render(ui);
        });

        // Procedure details 
        if let ProcedureStatus::Running { proc_idx, phase_idx } = state.proc_state {
            ui.group(|ui| {
                let procedure_name = &self.procedures[proc_idx].name;
                let phase_name = &self.procedures[proc_idx].phases[phase_idx].name;

                ui.label(format!("Executing: {}", procedure_name));
                ui.colored_label(Color32::LIGHT_BLUE, format!("Current phase: {} - {}", phase_idx + 1, phase_name));
            });
        } else if let Some(idx) = self.current_procedure {
            ui.label(format!("Selected: {}", self.procedures[idx].name));
        }
    }
}