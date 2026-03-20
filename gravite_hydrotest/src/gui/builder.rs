use crate::{config::AppConfig, gui::composite::{ButtonModule, Component, GroupModule}};

pub struct GroupBuilder {
    name: String,
    children: Vec<Box<dyn Component>>
}

impl GroupBuilder {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), children: vec![] }
    }
    
    fn button(mut self, name: &str, code: u8) -> Self {
        self.children.push(Box::new(ButtonModule::new(name, code)));
        self
    }

    fn group(mut self, group: GroupModule) -> Self {
        self.children.push(Box::new(group));
        self
    }

    fn build(self) -> GroupModule {
        GroupModule {
            name: self.name,
            children: self.children,
        }
    }

    pub fn build_from_cfg(config: &AppConfig) -> GroupModule {
        let mut root = GroupBuilder::new("Actuators control");

        for section in &config.sections {
            let mut section_builder = GroupBuilder::new(&section.name);

            for btn in &section.buttons {
                section_builder = section_builder.button(&btn.name, btn.code);
            }

            root = root.group(section_builder.build());
        }

        root.build()
    }
}