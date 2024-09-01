use crate::models::entity::Entity;

pub mod naming_conventions;
pub mod template_variables;
pub mod models;
pub mod filters;
pub mod attributes;
pub mod composite_keys;
pub mod contents;


pub trait CodeGen {
    fn generate_code(&self, entity: &Entity, entities: &Vec<Entity>) -> anyhow::Result<String>;
}
