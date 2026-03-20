use crate::gui::composite::{Component, GroupModule};

pub struct ActuatorsPage {
    pub root: GroupModule,
}

impl ActuatorsPage {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Vec<u8> {
        self.root.show(ui)
    }
}