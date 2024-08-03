use serde::{Deserialize, Serialize};

use super::entity::Entity;

pub type EntityName = String;
#[derive(Serialize, Deserialize)]
pub struct DomainDrivenRequest {
    pub service_name: String,
    pub entities: Vec<Entity>,
}

impl DomainDrivenRequest {
    /**
     * Possible constraints:
     * - All attributes used as primary key, filter by, unique attributes, or foreign keys must be present
     * - Primary key must be in the attributes
     * - If there are unique attributes, they need to be present in filter_by
     * - If an attribute is unique, it cannot be used as a sub attribute in filter_by
     * For Example:
     * name is a unique attribute
     * It is not possible to have a filter_by with: [id, name], since name is already unique
     */
    pub fn validate_request(&self) -> anyhow::Result<()> {
        for entity in &self.entities {
            entity.validate_request(self.entities.clone())?;
        }
        Ok(())
    }
}
