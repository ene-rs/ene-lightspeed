use models::StringWithNamingConvention;

use crate::models::entity::Entity;

pub mod naming_conventions;
pub mod template_variables;

pub mod entity_codegen;
pub mod models;

pub trait CodeGen {
    fn generate_code(&self, entity: &Entity, entities: &Vec<Entity>) -> String;
}