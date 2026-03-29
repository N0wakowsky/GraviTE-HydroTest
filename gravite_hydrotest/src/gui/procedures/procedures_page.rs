use crate::gui::components::ActionButton;
use crate::gui::components::AppMessage;
use crate::gui::components::GuiComponent;
use crate::gui::components::LayoutType;

use crate::gui::components::GroupModule;
use crate::config::AppConfig;
use crate::config::ProceduresConfig;

pub struct ProcedurePage {
    pub root: GroupModule,
}

impl ProcedurePage {
    pub fn new(config: &AppConfig) -> Self {
        Self { root: ProcedurePage::build_from_cfg(&config.procedures)}
    }

    pub fn build_from_cfg(config: &[ProceduresConfig]) -> GroupModule {
        let mut top_layout = GroupModule::new("Procedures", LayoutType::Vertical);

        for procedure in config {
            let button = ActionButton::new(
                procedure.name.clone(), 
                AppMessage::StartProcedure(procedure.name.clone())
            );
            
            top_layout.children.push(GuiComponent::Button(button));
        }

        let stop_button = ActionButton::new(
            "Abort Procedure".to_string(), 
            AppMessage::AbortProcedure
        );

        top_layout.children.push(GuiComponent::Button(stop_button));

        top_layout.build()
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<AppMessage> {
        self.root.show(ui)
    }
}