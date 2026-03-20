use eframe::egui;

pub struct ProceduresPage {
    pub full_procedure_run: bool,
    pub autotest_run: bool,
}

impl ProceduresPage {
    pub fn new() -> Self {
        Self { full_procedure_run: false, autotest_run: false }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Test procedures");
        ui.separator();

        if ui.button("Full procedure").clicked() {
            self.full_procedure_run = true;
        }

        if ui.button("Autotest").clicked() {
            self.autotest_run = true;
        }
    }
}