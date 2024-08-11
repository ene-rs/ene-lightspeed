use crate::models::entity::{Entity, FilterByAttributes, MostSpecificFilterAttributes};

use super::CodeGen;
use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToTrainCase, ToUpperCamelCase,
};
#[derive(Debug, Clone)]
pub struct EntityTemplate {
    pub entity_content: Vec<EntityTemplateContent>,
}

impl CodeGen for EntityTemplate {
    fn generate_code(&self, entity: &Entity, entities: &Vec<Entity>) -> anyhow::Result<String> {
        self.entity_content
            .iter()
            .map(|entity_content| entity_content.generate_code(entity, entities))
            .collect::<anyhow::Result<Vec<String>>>()
            .map(|entity_contents| entity_contents.join(""))
    }
}

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
}

impl CodeGen for EntityTemplateContent {
    fn generate_code(&self, entity: &Entity, entities: &Vec<Entity>) -> anyhow::Result<String> {
        match self {
            Self::Content(content) => content.generate_code(entity, entities),
            Self::AttributeRep(attribute_rep) => attribute_rep.generate_code(entity, entities),
            Self::FilterRep(filter_rep) => filter_rep.generate_code(entity, entities),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AttributeRep {
    NamesAndTypesPresent {
        prefix: Content,
        template_attribute_name: StringWithNamingConvention,
        attribute_name_and_type_separator: Content,
        template_attribute_type: StringWithNamingConvention,
        attributes_separator: Content,
        suffix: Content,
    },
    NamesPresent {
        prefix: Content,
        template_attribute_name: StringWithNamingConvention,
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
                let prefix = prefix.generate_code(entity, entities)?;
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
                let suffix = suffix.generate_code(entity, entities)?;
                Ok(format!("{}{}{}", prefix, attributes, suffix))
            }
            Self::NamesPresent {
                prefix,
                template_attribute_name,
                attributes_separator,
                suffix,
            } => {
                let prefix = prefix.generate_code(entity, entities)?;
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
                let suffix = suffix.generate_code(entity, entities)?;
                Ok(format!("{}{}{}", prefix, attributes, suffix))
            }
            Self::NamesTwicePresent {
                prefix,
                template_attribute_name,
                attribute_name_twice_separator,
                attributes_separator,
                suffix,
            } => {
                let prefix = prefix.generate_code(entity, entities)?;
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
                let suffix = suffix.generate_code(entity, entities)?;
                Ok(format!("{}{}{}", prefix, attributes, suffix))
            }
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
    pub fn generate_code(
        &self,
        filter_by_attributes: &FilterByAttributes,
        entity: &Entity,
        entities: &Vec<Entity>,
    ) -> anyhow::Result<String> {
        match self {
            Self::Content(content) => content.generate_code(entity, entities),
            Self::MostSpecificFilterAttributeName(naming_convention) => {
                let (most_specific_filter_attribute_name, _) =
                    filter_by_attributes.get_most_specific_filter_attribute();
                Ok(naming_convention.to_naming_convention(&most_specific_filter_attribute_name))
            }
            Self::MostSpecificFilterAttributeType(naming_convention) => {
                let (_, most_specific_filter_attribute_type) =
                    filter_by_attributes.get_most_specific_filter_attribute();
                Ok(naming_convention.to_naming_convention(
                    &most_specific_filter_attribute_type.get_attribute_type(entities)?,
                ))
            }
        }
    }
}

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
    ContentWithMSFA(ContentWithMSFA),
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
            Self::ContentWithMSFA(content_with_msfa) => {
                content_with_msfa.generate_code(filter_by_attributes, entity, entities)
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
        prefix: Content,
        template_attribute_name: StringWithNamingConvention,
        attribute_name_and_type_separator: Content,
        template_attribute_type: StringWithNamingConvention,
        attributes_separator: Content,
        suffix: Content,
    },
    NamesPresent {
        prefix: Content,
        template_attribute_name: StringWithNamingConvention,
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
                let prefix = prefix.generate_code(&entity, &entities)?;
                let attributes = filter_by_attributes
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
                let attributes = filter_by_attributes
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
                let attributes = filter_by_attributes
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

#[derive(Debug, Clone)]
pub enum StringWithNamingConvention {
    KebabCase,
    LowerCamelCase,
    UpperCamelCase,
    ShoutyKebabCase,
    ShoutySnakeCase,
    SnakeCase,
    TitleCase,
    TrainCase,
}

impl StringWithNamingConvention {
    pub fn to_naming_convention(&self, value: &str) -> String {
        match self {
            Self::KebabCase => value.to_kebab_case(),
            Self::LowerCamelCase => value.to_lower_camel_case(),
            Self::UpperCamelCase => value.to_upper_camel_case(),
            Self::ShoutyKebabCase => value.to_shouty_kebab_case(),
            Self::ShoutySnakeCase => value.to_shouty_snake_case(),
            Self::SnakeCase => value.to_snake_case(),
            Self::TitleCase => value.to_title_case(),
            Self::TrainCase => value.to_train_case(),
        }
    }
}


// region: tests
#[cfg(test)]
mod tests {
    use crate::{codegen::CodeGen, models::entity::{AttributeType, Entity, ForeignKey, Semantics}, parsers::template_parser::TemplateParser};

    fn prepare_entities() -> Vec<Entity> {
         let car = Entity {
            name: "Car".to_string(),
            semantics: Semantics {
                plural: "Cars".to_string(),
            },
            attributes: vec![
                ("id".to_string(), AttributeType::Type("Uuid".to_string())),
                (
                    "name".to_string(),
                    AttributeType::Type("String".to_string()),
                ),
                (
                    "model".to_string(),
                    AttributeType::Type("String".to_string()),
                ),
                (
                    "manufacturer".to_string(),
                    AttributeType::Type("String".to_string()),
                ),
                ("year".to_string(), AttributeType::Type("i32".to_string())),
                (
                    "owner".to_string(),
                    AttributeType::ForeignKey(ForeignKey {
                        entity_name: "Person".to_string(),
                        attribute_name: "id".to_string(),
                    }),
                ),
            ]
            .into_iter()
            .collect(),
            primary_key: "id".to_string(),
            filter_by: vec![
                vec!["id".to_string()],
                vec!["name".to_string(), "model".to_string()],
                vec!["manufacturer".to_string()],
                vec!["year".to_string()],
                vec!["owner".to_string()],
            ],
            unique_attributes: vec![vec!["id".to_string()], vec!["name".to_string()]]
                .into_iter()
                .collect(),
        };
        let person = Entity {
            name: "Person".to_string(),
            semantics: Semantics {
                plural: "People".to_string(),
            },
            attributes: vec![
                ("id".to_string(), AttributeType::Type("Uuid".to_string())),
                (
                    "name".to_string(),
                    AttributeType::Type("String".to_string()),
                ),
                ("age".to_string(), AttributeType::Type("i32".to_string())),
            ]
            .into_iter()
            .collect(),
            primary_key: "id".to_string(),
            filter_by: vec![
                vec!["id".to_string()],
                vec!["name".to_string()],
                vec!["age".to_string()],
            ],
            unique_attributes: vec![vec!["id".to_string()], vec!["name".to_string()]]
                .into_iter()
                .collect(),
        };
        let entities = vec![car.clone(), person];
        entities
    }

    #[test]
    fn test_entity_code_gen() {
        let entity_model_test = std::fs::read_to_string("/Users/abdullahsabaaallil/Desktop/ene-lightspeed/template-rs/src/models/entity_name.rs").unwrap();
        let parsed_template = TemplateParser::parse_template(&entity_model_test).unwrap();
        let entities = prepare_entities();
        let generated_code = parsed_template.generate_code(&entities[0], &entities).unwrap();
        std::fs::write("test_output/entity_test.rs", generated_code.clone())
            .map_err(|e| {
                println!("Error: {:?}", e);
                e
            })
            .unwrap();
        insta::assert_debug_snapshot!(generated_code);
    }

    #[test]
    fn test_controller_code_gen() {
        let controller_test = std::fs::read_to_string("/Users/abdullahsabaaallil/Desktop/ene-lightspeed/template-rs/src/controllers/entity_controller.rs").unwrap();
        let parsed_template = TemplateParser::parse_template(&controller_test).unwrap();
        let entities = prepare_entities();
        let generated_code = parsed_template.generate_code(&entities[0], &entities).unwrap();
        std::fs::write("test_output/entity_controller_test.rs", generated_code.clone())
            .map_err(|e| {
                println!("Error: {:?}", e);
                e
            })
            .unwrap();
        insta::assert_debug_snapshot!(generated_code);
    }


}