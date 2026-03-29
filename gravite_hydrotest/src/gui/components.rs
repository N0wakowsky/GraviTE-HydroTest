/*
Components file is suposed to derive:
    - structures for each page to build its contents from

    GroupModule - top structure containing section name and buttons
*/

pub enum LayoutType {
    Vertical,
    Horizontal,
}

// All actions posible for button to communicate
#[derive(Clone)]
pub enum AppMessage {
    ToggleActuator(u8),
    StartProcedure(String),
    AbortProcedure,
    ConnectSerial { port: String, baud: u32 },
    DisconnectSerial,
}

// Button status
#[derive(Clone)]
pub enum ButtonStyle {
    Active,
    Inactive,
    NoEcho,
}

// Button struct
pub struct ActionButton {
    pub label: String,
    pub action: AppMessage,
    pub style: ButtonStyle,
}

impl ActionButton {
    pub fn new(label: String, action: AppMessage) -> Self {
        Self { label: label, action, style: ButtonStyle::NoEcho }
    }

    pub fn reset_status(&mut self) {
        self.style = ButtonStyle::NoEcho;
    }

    pub fn set_status(&mut self, active: bool) {
        self.style = if active {
            ButtonStyle::Active
        } else {
            ButtonStyle::Inactive
        };
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<AppMessage> {
        // Kolorowanie przycisku na podstawie jego stanu z echa
        let color = match self.style {
            ButtonStyle::Active => egui::Color32::from_rgb(50, 200, 50),
            ButtonStyle::Inactive => egui::Color32::from_rgb(200, 50, 50),
            ButtonStyle::NoEcho => egui::Color32::GRAY,
        };

        let button = egui::Button::new(&self.label).fill(color);
        
        if ui.add(button).clicked() {
            return Some(self.action.clone());
        }
        None
    }
}


// Structure for gruping all the buttons
pub enum GuiComponent {
    Button(ActionButton),
    Group(GroupModule),
}

impl GuiComponent {
    pub fn set_status_by_code(&mut self, target_code: u8, active: bool) {
        match self {
            GuiComponent::Button(btn) => {
                // Sprawdzamy, czy akcja przypisana do przycisku zawiera szukany kod
                if let AppMessage::ToggleActuator(code) = btn.action {
                    if code == target_code {
                        btn.set_status(active);
                    }
                }
            }
            GuiComponent::Group(grp) => {
                // Jeśli to grupa, delegujemy zadanie do jej dzieci
                grp.set_status_by_code(target_code, active);
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<AppMessage> {
        match self {
            GuiComponent::Button(btn) => btn.show(ui), // ActionButton::show już zwraca Option<AppMessage>
            GuiComponent::Group(grp) => grp.show(ui),
        }
    }

    pub fn reset_status(&mut self) {
        match self {
            GuiComponent::Button(btn) => btn.reset_status(),
            GuiComponent::Group(grp) => grp.reset_status(),
        }
    }
}



pub struct GroupModule {
    pub module_name: String,
    pub children: Vec<GuiComponent>,
    pub layout: LayoutType,
}

impl GroupModule {
    pub fn new(module_name: &str, layout: LayoutType) -> Self {
        Self { module_name: module_name.to_string(), children: vec![], layout}
    }

    pub fn button(mut self, name: &str, code: u8) -> Self {
        let btn = ActionButton::new(name.to_string(), AppMessage::ToggleActuator(code));
        self.children.push(GuiComponent::Button(btn));
        self
    }

    pub fn group(mut self, item: GroupModule) -> Self {
        self.children.push(GuiComponent::Group(item));
        self
    }

    pub fn build(self) -> GroupModule {
        self
    }

    pub fn set_status_by_code(&mut self, target_code: u8, active: bool) {
        for child in &mut self.children {
            child.set_status_by_code(target_code, active);
        }
    }

    pub fn reset_status(&mut self) {
        for child in &mut self.children {
            child.reset_status();
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<AppMessage> {
        let mut clicked_msg = None;
        ui.push_id(&self.module_name, |ui| {
            ui.group(|ui| {
                ui.label(egui::RichText::new(&self.module_name).strong());
                ui.separator();
                
                // Zamiast zmiennej layout_func, używamy match do wywołania layoutu
                match self.layout {
                    LayoutType::Vertical => {
                        ui.vertical(|ui| {
                            for child in &mut self.children {
                                if let Some(msg) = child.show(ui) {
                                    clicked_msg = Some(msg);
                                }
                            }
                        });
                    }
                    LayoutType::Horizontal => {
                        ui.horizontal_wrapped(|ui| {
                            for child in &mut self.children {
                                if let Some(msg) = child.show(ui) {
                                    clicked_msg = Some(msg);
                                }
                            }
                        });
                    }
                }
            });
        });
        clicked_msg
    }
}