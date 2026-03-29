use crate::{config::AppConfig, gui::components::GroupModule};
use crate::gui::components::LayoutType;
use crate::config::ActuatorsConfig;
use crate::gui::components::AppMessage;

pub struct ActuatorsPage {
    pub root: GroupModule,
}

impl ActuatorsPage {
    pub fn new(config: &AppConfig) -> Self {
        Self { root: Self::build_from_cfg(&config.actuators) }
    }


    fn build_from_cfg(config: &[ActuatorsConfig]) -> GroupModule {
        let mut top_layout = GroupModule::new("Actuator Control", LayoutType::Vertical);

        for section in config {
            let mut section_builder = GroupModule::new(&section.module, LayoutType::Horizontal);

            for btn in &section.buttons {
                section_builder = section_builder.button(&btn.name, btn.code);
            }

            top_layout = top_layout.group(section_builder.build());
        }

        top_layout.build()
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<AppMessage> {
        self.root.show(ui)
    }
}