use crate::models::entity::{Entity, RetrieveMostSpecificAttribute};

use super::{attributes::AttributeRep, composite_keys::CompositeKeyRep, filters::FilterRep, models::StringWithNamingConvention, CodeGen};


#[derive(Debug, Clone)]
pub enum Content {
    Static(String),
    EntityName(StringWithNamingConvention),
    EntityPlural(StringWithNamingConvention),
    PrimaryKey(StringWithNamingConvention),
}

impl CodeGen for Content {
    fn generate_code(&self, entity: &Entity, _entities: &Vec<Entity>) -> anyhow::Result<String> {
        Ok(match self {
            Self::Static(value) => value.clone(),
            Self::EntityName(naming_convention) => {
                naming_convention.to_naming_convention(&entity.name)
            }
            Self::EntityPlural(naming_convention) => {
                naming_convention.to_naming_convention(&entity.semantics.plural)
            }
            Self::PrimaryKey(naming_convention) => {
                naming_convention.to_naming_convention(&entity.primary_key)
            }
        })
    }
}

#[derive(Debug, Clone)]
pub enum EntityTemplateContent {
    Content(Content),
    AttributeRep(AttributeRep),
    FilterRep(FilterRep),
    CompositeKeyRep(CompositeKeyRep)
}

impl CodeGen for EntityTemplateContent {
    fn generate_code(&self, entity: &Entity, entities: &Vec<Entity>) -> anyhow::Result<String> {
        match self {
            Self::Content(content) => content.generate_code(entity, entities),
            Self::AttributeRep(attribute_rep) => attribute_rep.generate_code(entity, entities),
            Self::FilterRep(filter_rep) => filter_rep.generate_code(entity, entities),
            Self::CompositeKeyRep(composite_key_rep) => composite_key_rep.generate_code(entity, entities)
        }
    }
}


#[derive(Debug, Clone)]
pub enum ContentWithMSFA {
    Content(Content),
    MostSpecificFilterAttributeName(StringWithNamingConvention),
    MostSpecificFilterAttributeType(StringWithNamingConvention),
}

impl ContentWithMSFA {
    pub fn generate_code<T: RetrieveMostSpecificAttribute>(
        &self,
        filter_by_attributes: &T,
        entity: &Entity,
        entities: &Vec<Entity>,
    ) -> anyhow::Result<String> {
        match self {
            Self::Content(content) => content.generate_code(entity, entities),
            Self::MostSpecificFilterAttributeName(naming_convention) => {
                let (most_specific_filter_attribute_name, _) =
                    filter_by_attributes.get_most_specific_attribute();
                Ok(naming_convention.to_naming_convention(&most_specific_filter_attribute_name))
            }
            Self::MostSpecificFilterAttributeType(naming_convention) => {
                let (_, most_specific_filter_attribute_type) =
                    filter_by_attributes.get_most_specific_attribute();
                Ok(naming_convention.to_naming_convention(
                    &most_specific_filter_attribute_type.get_attribute_type(entities)?,
                ))
            }
        }
    }
}