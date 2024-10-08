use crate::models::entity::{Entity, FilterByAttributes};

use super::{contents::{Content, ContentWithMSA}, models::StringWithNamingConvention, CodeGen};


#[derive(Debug, Clone)]
pub enum FilterRep {
    ConsecutiveFilters { filter: Filter },
    SeparatedFilters { filter: Filter, separator: Content },
}

impl CodeGen for FilterRep {
    fn generate_code(&self, entity: &Entity, entities: &Vec<Entity>) -> anyhow::Result<String> {
        match self {
            Self::ConsecutiveFilters { filter } => entity
                .get_filter_by_attributes()
                .iter()
                .map(|filter_by_attributes| {
                    filter.generate_code(filter_by_attributes, entity, entities)
                })
                .collect::<anyhow::Result<Vec<String>>>()
                .map(|filters| filters.join("")),
            Self::SeparatedFilters { filter, separator } => entity
                .get_filter_by_attributes()
                .iter()
                .map(|filter_by_attributes| {
                    filter.generate_code(filter_by_attributes, entity, entities)
                })
                .collect::<anyhow::Result<Vec<String>>>()
                .map(|filters| -> anyhow::Result<String> {
                    Ok(filters.join(separator.generate_code(entity, entities)?.as_str()))
                })?,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub content: Vec<FilterBody>,
}

impl Filter {
    pub fn generate_code(
        &self,
        filter_by_attributes: &FilterByAttributes,
        entity: &Entity,
        entities: &Vec<Entity>,
    ) -> anyhow::Result<String> {
        self.content
            .iter()
            .map(|filter_body| -> anyhow::Result<String> {
                filter_body.generate_code(filter_by_attributes, entity, entities)
            })
            .collect::<anyhow::Result<Vec<String>>>()
            .map(|filter_bodies| filter_bodies.join(""))
    }
}
#[derive(Debug, Clone)]
pub enum FilterBody {
    ContentWithMSA(ContentWithMSA),
    FilterAttributeRep(FilterAttributeRep),
}

impl FilterBody {
    pub fn generate_code(
        &self,
        filter_by_attributes: &FilterByAttributes,
        entity: &Entity,
        entities: &Vec<Entity>,
    ) -> anyhow::Result<String> {
        match self {
            Self::ContentWithMSA(content_with_msa) => {
                content_with_msa.generate_code(&filter_by_attributes, entity, entities)
            }
            Self::FilterAttributeRep(filter_attribute_rep) => {
                filter_attribute_rep.generate_code(filter_by_attributes, entity, entities)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum FilterAttributeRep {
    NamesAndTypesPresent {
        prefix: Option<ContentWithMSA>,
        template_attribute_name: StringWithNamingConvention,
        attribute_name_and_type_separator: ContentWithMSA,
        template_attribute_type: StringWithNamingConvention,
        attributes_separator: ContentWithMSA,
        suffix: Option<ContentWithMSA>,
    },
    NamesPresent {
        prefix: Option<ContentWithMSA>,
        template_attribute_name: StringWithNamingConvention,
        attributes_separator: ContentWithMSA,
        suffix: Option<ContentWithMSA>,
    },
    NamesTwicePresent {
        prefix: Option<ContentWithMSA>,
        template_attribute_name: StringWithNamingConvention,
        attribute_name_twice_separator: ContentWithMSA,
        attributes_separator: ContentWithMSA,
        suffix: Option<ContentWithMSA>,
    },
}

impl FilterAttributeRep {
    pub fn generate_code(
        &self,
        filter_by_attributes: &FilterByAttributes,
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
                let prefix = prefix
                    .as_ref()
                    .map(|prefix| -> anyhow::Result<String> {
                        prefix.generate_code(&filter_by_attributes, &entity, &entities)
                    })
                    .transpose()?
                    .unwrap_or_default();
                let attributes = filter_by_attributes
                    .iter()
                    .map(
                        |(entity_attribute_name, entity_attribute_type)| -> anyhow::Result<String> {
                            let attribute_name_and_type_separator =
                                attribute_name_and_type_separator
                                    .generate_code(&filter_by_attributes, &entity, &entities)?;
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
                            .generate_code(&filter_by_attributes, &entity, &entities)?
                            .as_str(),
                    );
                let suffix = suffix
                    .as_ref()
                    .map(|suffix| -> anyhow::Result<String> {
                        suffix.generate_code(&filter_by_attributes, &entity, &entities)
                    })
                    .transpose()?
                    .unwrap_or_default();
                Ok(format!("{}{}{}", prefix, attributes, suffix))
            }
            Self::NamesPresent {
                prefix,
                template_attribute_name,
                attributes_separator,
                suffix,
            } => {
                let prefix = prefix
                    .as_ref()
                    .map(|prefix| -> anyhow::Result<String> {
                        prefix.generate_code(&filter_by_attributes, &entity, &entities)
                    })
                    .transpose()?
                    .unwrap_or_default();
                let attributes = filter_by_attributes
                    .iter()
                    .map(|(entity_attribute_name, _)| -> anyhow::Result<String> {
                        Ok(template_attribute_name.to_naming_convention(entity_attribute_name))
                    })
                    .collect::<anyhow::Result<Vec<String>>>()?
                    .join(
                        attributes_separator
                            .generate_code(&filter_by_attributes, &entity, &entities)?
                            .as_str(),
                    );
                let suffix = suffix
                    .as_ref()
                    .map(|suffix| -> anyhow::Result<String> {
                        suffix.generate_code(&filter_by_attributes, &entity, &entities)
                    })
                    .transpose()?
                    .unwrap_or_default();
                Ok(format!("{}{}{}", prefix, attributes, suffix))
            }
            Self::NamesTwicePresent {
                prefix,
                template_attribute_name,
                attribute_name_twice_separator,
                attributes_separator,
                suffix,
            } => {
                let prefix = prefix
                    .as_ref()
                    .map(|prefix| -> anyhow::Result<String> {
                        prefix.generate_code(&filter_by_attributes, &entity, &entities)
                    })
                    .transpose()?
                    .unwrap_or_default();
                let attributes = filter_by_attributes
                    .iter()
                    .map(|(entity_attribute_name, _)| -> anyhow::Result<String> {
                        Ok(format!(
                            "{}{}{}",
                            template_attribute_name.to_naming_convention(entity_attribute_name),
                            attribute_name_twice_separator.generate_code(&filter_by_attributes, &entity, &entities)?,
                            template_attribute_name.to_naming_convention(entity_attribute_name)
                        ))
                    })
                    .collect::<anyhow::Result<Vec<String>>>()?
                    .join(
                        attributes_separator
                            .generate_code(&filter_by_attributes, &entity, &entities)?
                            .as_str(),
                    );
                let suffix = suffix
                    .as_ref()
                    .map(|suffix| -> anyhow::Result<String> {
                        suffix.generate_code(&filter_by_attributes, &entity, &entities)
                    })
                    .transpose()?
                    .unwrap_or_default();
                Ok(format!("{}{}{}", prefix, attributes, suffix))
            }
        }
    }
}