use crate::models::entity::Entity;

pub mod naming_conventions;
pub mod template_variables;
pub mod models;

pub trait CodeGen {
    fn generate_code(&self, entity: &Entity, entities: &Vec<Entity>) -> anyhow::Result<String>;
}
