use crate::models::entity::Entity;

use super::{contents::{Content, ContentWithMSFA}, models::StringWithNamingConvention, CodeGen};


#[derive(Debug, Clone)]
pub enum CompositeKeyRep {
    ConsecutiveCompositeKeys { composite_key: CompositeKey },
    SeparatedCompositeKeys { composite_key: CompositeKey, separator: Content },
}

impl CodeGen for CompositeKeyRep {
    fn generate_code(&self, entity: &Entity, entities: &Vec<Entity>) -> anyhow::Result<String> {
        match self {
            Self::ConsecutiveCompositeKeys { composite_key } => entity
                .get_composite_keys()
                .iter()
                .map(|composite_key_attributes| {
                    composite_key.generate_code(composite_key_attributes, entity, entities)
                })
                .collect::<anyhow::Result<Vec<String>>>()
                .map(|filters| filters.join("")),
            Self::SeparatedCompositeKeys { composite_key, separator } => entity
                .get_composite_keys()
                .iter()
                .map(|composite_key_attributes| {
                    composite_key.generate_code(composite_key_attributes, entity, entities)
                })
                .collect::<anyhow::Result<Vec<String>>>()
                .map(|comosite_keys| -> anyhow::Result<String> {
                    Ok(comosite_keys.join(separator.generate_code(entity, entities)?.as_str()))
                })?,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompositeKey {
    pub content: Vec<CompositeKeyBody>,
}

impl CompositeKey {
    pub fn generate_code(
        &self,
        composite_key_attributes: &crate::models::entity::CompositeKey,
        entity: &Entity,
        entities: &Vec<Entity>,
    ) -> anyhow::Result<String> {
        self.content
            .iter()
            .map(|composite_key_body| -> anyhow::Result<String> {
                composite_key_body.generate_code(composite_key_attributes, entity, entities)
            })
            .collect::<anyhow::Result<Vec<String>>>()
            .map(|composite_key_bodies| composite_key_bodies.join(""))
    }
}

#[derive(Debug, Clone)]
pub enum CompositeKeyBody {
    ContentWithMSFA(ContentWithMSFA),
    CompositeKeyAttributeRep(CompositeKeyAttributeRep),
}

impl CompositeKeyBody {
    pub fn generate_code(
        &self,
        composite_key: &crate::models::entity::CompositeKey,
        entity: &Entity,
        entities: &Vec<Entity>,
    ) -> anyhow::Result<String> {
        match self {
            Self::ContentWithMSFA(content_with_msfa) => {
                content_with_msfa.generate_code(&composite_key, entity, entities)
            }
            Self::CompositeKeyAttributeRep(composite_key_attribute_rep) => {
                composite_key_attribute_rep.generate_code(composite_key, entity, entities)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum CompositeKeyAttributeRep {
    NamesPresent {
        prefix: Content,
        template_attribute_name: StringWithNamingConvention,
        attributes_separator: Content,
        suffix: Content,
    },
    NamesAndTypesPresent {
        prefix: Content,
        template_attribute_name: StringWithNamingConvention,
        attribute_name_and_type_separator: Content,
        template_attribute_type: StringWithNamingConvention,
        attributes_separator: Content,
        suffix: Content,
    },
    NamesTwicePresent {
        prefix: Content,
        template_attribute_name: StringWithNamingConvention,
        attribute_name_twice_separator: Content,
        attributes_separator: Content,
        suffix: Content,
    },
}

impl CompositeKeyAttributeRep {
    pub fn generate_code(
        &self,
        composite_key: &crate::models::entity::CompositeKey,
        entity: &Entity,
        entities: &Vec<Entity>,
    ) -> anyhow::Result<String> {
        match self {
            Self::NamesAndTypesPresent {
                prefix,
                template_attribute_name,
                attribute_name_and_type_separator,
                template_attribute_type,
                attributes_separator,
                suffix,
            } => {
                let prefix = prefix.generate_code(&entity, &entities)?;
                let attributes = composite_key
                    .iter()
                    .map(
                        |(entity_attribute_name, entity_attribute_type)| -> anyhow::Result<String> {
                            let attribute_name_and_type_separator =
                                attribute_name_and_type_separator
                                    .generate_code(&entity, &entities)?;
                            let attribute_type = template_attribute_type.to_naming_convention(
                                &entity_attribute_type.get_attribute_type(&entities)?,
                            );
                            Ok(format!(
                                "{}{}{}",
                                template_attribute_name.to_naming_convention(entity_attribute_name),
                                attribute_name_and_type_separator,
                                attribute_type
                            ))
                        },
                    )
                    .collect::<anyhow::Result<Vec<String>>>()?
                    .join(
                        attributes_separator
                            .generate_code(&entity, &entities)?
                            .as_str(),
                    );
                let suffix = suffix.generate_code(&entity, &entities)?;
                Ok(format!("{}{}{}", prefix, attributes, suffix))
            }
            Self::NamesPresent {
                prefix,
                template_attribute_name,
                attributes_separator,
                suffix,
            } => {
                let prefix = prefix.generate_code(&entity, &entities)?;
                let attributes = composite_key
                    .iter()
                    .map(|(entity_attribute_name, _)| -> anyhow::Result<String> {
                        Ok(template_attribute_name.to_naming_convention(entity_attribute_name))
                    })
                    .collect::<anyhow::Result<Vec<String>>>()?
                    .join(
                        attributes_separator
                            .generate_code(&entity, &entities)?
                            .as_str(),
                    );
                let suffix = suffix.generate_code(&entity, &entities)?;
                Ok(format!("{}{}{}", prefix, attributes, suffix))
            }
            Self::NamesTwicePresent {
                prefix,
                template_attribute_name,
                attribute_name_twice_separator,
                attributes_separator,
                suffix,
            } => {
                let prefix = prefix.generate_code(&entity, &entities)?;
                let attributes = composite_key
                    .iter()
                    .map(|(entity_attribute_name, _)| -> anyhow::Result<String> {
                        Ok(format!(
                            "{}{}{}",
                            template_attribute_name.to_naming_convention(entity_attribute_name),
                            attribute_name_twice_separator.generate_code(&entity, &entities)?,
                            template_attribute_name.to_naming_convention(entity_attribute_name)
                        ))
                    })
                    .collect::<anyhow::Result<Vec<String>>>()?
                    .join(
                        attributes_separator
                            .generate_code(&entity, &entities)?
                            .as_str(),
                    );
                let suffix = suffix.generate_code(&entity, &entities)?;
                Ok(format!("{}{}{}", prefix, attributes, suffix))
            }
        }
    }
}