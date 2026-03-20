use eframe::egui;

// konstrukcja przycisków
#[derive(Clone)]
pub enum ButtonState {
    Active,
    Inactive,
    NoEcho,
}

pub struct ButtonModule {
    pub name: String,
    pub peripheral_code: u8,
    pub state: ButtonState,
}

impl ButtonModule {
    pub fn new(name: &str, code: u8) -> Self{
        Self {
            name: name.to_string(),
            peripheral_code: code,
            state: ButtonState::NoEcho,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<u8> {
        let color = match self.state {
            ButtonState::Active => egui::Color32::from_rgb(50, 200, 50),
            ButtonState::Inactive => egui::Color32::from_rgb(200, 50, 50),
            ButtonState::NoEcho => egui::Color32::from_rgb(120, 120, 120),
        };

        let btn = egui::Button::new(&self.name).fill(color);

        if ui.add(btn).clicked() {
            return Some(self.peripheral_code);
        }
        None
    }
}

// moduł skupiający przyciski

pub struct GroupModule {
    pub name: String,
    pub children: Vec<Box<dyn Component>>
}

pub trait Component {
    #[allow(dead_code)]
    fn name(&self) -> &str;
    fn show(&mut self, ui: &mut egui::Ui) -> Vec<u8>;
    fn update_state(&mut self, code: u8, state: ButtonState);
}

impl Component for ButtonModule {
    fn name(&self) -> &str {
        &self.name
    }
    fn show(&mut self, ui: &mut egui::Ui) -> Vec<u8> {
        self.show(ui).into_iter().collect()
    }
    fn update_state(&mut self, code: u8, state: ButtonState) {
        if self.peripheral_code == code {
            self.state = state;
        }
    }
}

impl Component for GroupModule {
    fn name(&self) -> &str {
        &self.name
    }
    fn show(&mut self, ui: &mut egui::Ui) -> Vec<u8> {
        let mut clicked = vec![];
        ui.push_id(&self.name, |ui| {
            ui.group(|ui| {
                ui.label(egui::RichText::new(&self.name).strong());
                ui.separator();
                for child in &mut self.children {
                    let codes = child.show(ui);
                    clicked.extend(codes);
                }
            });
        });
        clicked
    }
    fn update_state(&mut self, code: u8, state: ButtonState) {
        for child in &mut self.children {
            child.update_state(code, state.clone());
        }
    }
}