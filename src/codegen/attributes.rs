use crate::models::entity::Entity;

use super::{contents::Content, models::StringWithNamingConvention, CodeGen};


#[derive(Debug, Clone)]
pub enum AttributeRep {
    NamesAndTypesPresent {
        prefix: Option<Content>,
        template_attribute_name: StringWithNamingConvention,
        attribute_name_and_type_separator: Content,
        template_attribute_type: StringWithNamingConvention,
        attributes_separator: Content,
        suffix: Option<Content>,
    },
    NamesPresent {
        prefix: Option<Content>,
        template_attribute_name: StringWithNamingConvention,
        attributes_separator: Content,
        suffix: Option<Content>,
    },
    NamesTwicePresent {
        prefix: Option<Content>,
        template_attribute_name: StringWithNamingConvention,
        attribute_name_twice_separator: Content,
        attributes_separator: Content,
        suffix: Option<Content>,
    },
}

impl CodeGen for AttributeRep {
    fn generate_code(&self, entity: &Entity, entities: &Vec<Entity>) -> anyhow::Result<String> {
        match self {
            Self::NamesAndTypesPresent {
                prefix,
                template_attribute_name,
                attribute_name_and_type_separator,
                template_attribute_type,
                attributes_separator,
                suffix,
            } => {
                let prefix = prefix.as_ref().map(|prefix| -> anyhow::Result<String> { prefix.generate_code(entity, entities)}).transpose()?.unwrap_or_default();
                let attributes = entity
                    .attributes
                    .iter()
                    .map(
                        |(entity_attribute_name, entity_attribute_type)| -> anyhow::Result<String> {
                            let attribute_name_and_type_separator =
                                attribute_name_and_type_separator
                                    .generate_code(entity, entities)?;
                            let attribute_type = template_attribute_type.to_naming_convention(
                                &entity_attribute_type.get_attribute_type(entities)?,
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
                            .generate_code(entity, entities)?
                            .as_str(),
                    );
                let suffix = suffix.as_ref().map(|suffix| -> anyhow::Result<String> { suffix.generate_code(entity, entities)}).transpose()?.unwrap_or_default();
                Ok(format!("{}{}{}", prefix, attributes, suffix))
            }
            Self::NamesPresent {
                prefix,
                template_attribute_name,
                attributes_separator,
                suffix,
            } => {
                let prefix = prefix.as_ref().map(|prefix| -> anyhow::Result<String> { prefix.generate_code(entity, entities)}).transpose()?.unwrap_or_default();
                let attributes = entity
                    .attributes
                    .iter()
                    .map(|(entity_attribute_name, _)| -> anyhow::Result<String> {
                        Ok(template_attribute_name.to_naming_convention(entity_attribute_name))
                    })
                    .collect::<anyhow::Result<Vec<String>>>()?
                    .join(
                        attributes_separator
                            .generate_code(entity, entities)?
                            .as_str(),
                    );
                let suffix = suffix.as_ref().map(|suffix| -> anyhow::Result<String> { suffix.generate_code(entity, entities)}).transpose()?.unwrap_or_default();
                Ok(format!("{}{}{}", prefix, attributes, suffix))
            }
            Self::NamesTwicePresent {
                prefix,
                template_attribute_name,
                attribute_name_twice_separator,
                attributes_separator,
                suffix,
            } => {
                let prefix = prefix.as_ref().map(|prefix| -> anyhow::Result<String> { prefix.generate_code(entity, entities)}).transpose()?.unwrap_or_default();
                let attributes = entity
                    .attributes
                    .iter()
                    .map(|(entity_attribute_name, _)| -> anyhow::Result<String> {
                        Ok(format!(
                            "{}{}{}",
                            template_attribute_name.to_naming_convention(entity_attribute_name),
                            attribute_name_twice_separator.generate_code(entity, entities)?,
                            template_attribute_name.to_naming_convention(entity_attribute_name)
                        ))
                    })
                    .collect::<anyhow::Result<Vec<String>>>()?
                    .join(
                        attributes_separator
                            .generate_code(entity, entities)?
                            .as_str(),
                    );
                let suffix = suffix.as_ref().map(|suffix| -> anyhow::Result<String> { suffix.generate_code(entity, entities)}).transpose()?.unwrap_or_default();
                Ok(format!("{}{}{}", prefix, attributes, suffix))
            }
        }
    }
}