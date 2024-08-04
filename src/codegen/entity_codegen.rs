/* use crate::{
    codegen::template_variables::TemplateVariable,
    models::entity::{AttributeType, Entity, FilterByAttributes, ForeignKey},
    parsers::template_parser::{Filter, FilterContent, TemplateEntity, TemplateEntityContent},
};

use super::{
    naming_conventions::detect_naming_convention,
    template_variables::find_template_variable_in_content,
};
use crate::models::entity::MostSpecificFilterAttributes;
use anyhow::bail;

pub fn entity_codegen(
    entity: &Entity,
    entities: &Vec<Entity>,
    parsed_template: &TemplateEntity,
) -> anyhow::Result<String> {
    let mut code = String::new();
    for entity_content in &parsed_template.entity_content {
        match entity_content {
            TemplateEntityContent::Static { content, .. } => {
                code.push_str(content);
            }
            TemplateEntityContent::Attribute { content, .. } => {
                code.push_str(&attribute_code_gen(entity, entities, content)?);
            }
            TemplateEntityContent::Filter { filters } => {
                code.push_str(&filter_code_gen(entity, entities, filters)?);
            }
        }
    }

    let entity_name_plural =
        find_template_variable_in_content(&code, TemplateVariable::Entities);

    code = replace_variable(&code, entity_name_plural, &entity.semantics.plural);


    let entity_name = find_template_variable_in_content(&code, TemplateVariable::Entity);
    code = replace_variable(&code, entity_name, entity.name.as_str());


    Ok(code)
}

fn replace_variable(
    content: &str,
    mut variable_in_content: Vec<(usize, &str)>,
    variable: &str,
) -> String {
    if variable_in_content.is_empty() {
        content.to_string()
    } else {
        variable_in_content.sort_by(|(pos1, _), (pos2, _)| pos1.cmp(pos2));
        let mut res = String::new();
        let mut prev_pos = 0;
        for (pos, variable_str) in variable_in_content {
            let variable_naming_convention = detect_naming_convention(variable_str);
            // use the pos to change the variable in the content
            let variable_name =
                variable_naming_convention.transform_string_to_naming_convention(variable);
            let variable_name_len = variable_str.len();

            res.push_str(&content[prev_pos..pos]);
            res.push_str(&variable_name);
            prev_pos = pos + variable_name_len;
        }
        res.push_str(&content[prev_pos..]);
        res
    }
}

/// In the content: Find attribute_name, optionally attribute_type, attribute_name2, optionally attribute_type2
/// If attribute_name2 is present: get the separator between attribute_name and attribute_name2
/// If attribute_type is present: get the separator between attribute_name and attribute_type
/// Iterate through the entities attributes, and generate the code for each attribute
/// At least attribute_name and attribute_name2 must be present
fn attribute_code_gen(
    entity: &Entity,
    entities: &Vec<Entity>,
    content: &str,
) -> anyhow::Result<String> {
    let attribute_name: Vec<(usize, &str)> =
        find_template_variable_in_content(content, TemplateVariable::AttributeName);
    let attribute_type: Vec<(usize, &str)> =
        find_template_variable_in_content(content, TemplateVariable::AttributeType);
    let attribute_name2: Vec<(usize, &str)> =
        find_template_variable_in_content(content, TemplateVariable::Attribute2Name);
    let attribute_type2: Vec<(usize, &str)> =
        find_template_variable_in_content(content, TemplateVariable::Attribute2Type);

    match (attribute_name.as_slice(), attribute_type.as_slice(), attribute_name2.as_slice(), attribute_type2.as_slice()) {
        (
            [(attribute_name_idx, attribute_name)],
            [(attribute_type_idx, attribute_type)],
            [(attribute_name2_idx, _attribute_name2)],
            [(attribute_type2_idx, attribute_type2)]) => {
                let attribute_name_separator = content[attribute_name_idx + attribute_name.len()..*attribute_type_idx].to_string();
                let attributes_separator = content[attribute_type_idx + attribute_type.len()..*attribute_name2_idx].to_string();
                let attribute_name_naming_convention = detect_naming_convention(attribute_name);
                let attribute_type_naming_convention = detect_naming_convention(attribute_type);

                Ok(
                    content[..*attribute_name_idx].to_string() +
                    &entity.attributes.iter().map(|(attribute_name, attribute_type)| {
                    Ok(format!("{}{}{}", 
                    attribute_name_naming_convention.transform_string_to_naming_convention(attribute_name),
                    attribute_name_separator,
                    attribute_type_naming_convention.transform_string_to_naming_convention(&get_attribute_type(attribute_type, entities)?)))
                }).collect::<anyhow::Result<Vec<String>>>()?.join(&attributes_separator) +
                &content[*attribute_type2_idx + attribute_type2.len()..]
            )
            }
        (
            [(attribute_name_idx, attribute_name)],
            [],
            [(attribute_name2_idx, attribute_name2)],
            []
        ) => {
            let attributes_separator = content[attribute_name_idx + attribute_name.len()..*attribute_name2_idx].to_string();
            let attribute_name_naming_convention = detect_naming_convention(attribute_name);
            Ok(
                content[..*attribute_name_idx].to_string() +
                &entity.attributes.iter().map(|(attribute_name, _attribute_type)| {
                attribute_name_naming_convention.transform_string_to_naming_convention(attribute_name)
            }).collect::<Vec<String>>().join(&attributes_separator) +
                &content[*attribute_name2_idx + attribute_name2.len()..]
        )
        }
        (
            [(attribute_name_idx, attribute_name), (second_attribute_name_idx, second_attribute_name)],
            [],
            [(attribute_name2_idx, _attribute_name2), (second_attribute_name2_idx, second_attribute_name2)],
            []
        ) => {
            let attribute_names_separator = content[attribute_name_idx + attribute_name.len()..*second_attribute_name_idx].to_string();
            let attributes_separator = content[second_attribute_name_idx + second_attribute_name.len()..*attribute_name2_idx].to_string();
            let attribute_name_naming_convention = detect_naming_convention(attribute_name);
            let second_attribute_name_naming_convention = detect_naming_convention(second_attribute_name);
            Ok(
                content[..*attribute_name_idx].to_string() +
                &entity.attributes.iter().map(|(attribute_name, _attribute_type)| {
                Ok(format!("{}{}{}", 
                attribute_name_naming_convention.transform_string_to_naming_convention(attribute_name),
                attribute_names_separator,
                second_attribute_name_naming_convention.transform_string_to_naming_convention(attribute_name)))
                }).collect::<anyhow::Result<Vec<String>>>()?.join(&attributes_separator) +
                &content[*second_attribute_name2_idx + second_attribute_name2.len()..]
            )
        }
        _ => bail!("Invalid attribute content. Expected one of the following ways to define template attributes:TODO")
    }
}

fn filter_code_gen(
    entity: &Entity,
    entities: &Vec<Entity>,
    filters: &Vec<Filter>,
) -> anyhow::Result<String> {
    let mut code = String::new();
    for filter_by in entity.get_filter_by_attributes().iter() {
        let mut filter_code = String::new();
        for filter_content in content {
            match filter_content {
                FilterContent::Static { content, .. } => {
                    filter_code.push_str(content);
                }
                FilterContent::FilterAttribute { content, .. } => {
                    filter_code.push_str(&filter_attribute_code_gen(filter_by, entities, content)?);
                }
            }
        }
        let most_specific_filter_attribute_name = find_template_variable_in_content(
            &filter_code,
            TemplateVariable::MostSpecificFilterAttributeName,
        );
        filter_code = replace_variable(
            &filter_code,
            most_specific_filter_attribute_name,
            &filter_by.get_most_specific_filter_attribute().0,
        );
        let most_specific_filter_attribute_type = find_template_variable_in_content(
            &filter_code,
            TemplateVariable::MostSpecificFilterAttributeType,
        );
        match most_specific_filter_attribute_type.as_slice() {
            [(.., most_specific_filter_attribute_type_str)] => {
                let most_specific_filter_attribute_type_naming_convention =
                    detect_naming_convention(most_specific_filter_attribute_type_str);
                filter_code = replace_variable(
                    &filter_code,
                    most_specific_filter_attribute_type,
                    &most_specific_filter_attribute_type_naming_convention
                        .transform_string_to_naming_convention(&get_attribute_type(
                            &filter_by.get_most_specific_filter_attribute().1,
                            entities,
                        )?),
                );
            }
            _ => {}
        }
        code.push_str(&filter_code);

        code.push_str("\n"); // TODO: a new line for now between each filter
    }

    Ok(code)
}

fn filter_attribute_code_gen(
    filter: &FilterByAttributes,
    entities: &Vec<Entity>,
    content: &str,
) -> anyhow::Result<String> {
    let filter_attribute_name: Vec<(usize, &str)> =
        find_template_variable_in_content(content, TemplateVariable::FilterAttributeName);
    let filter_attribute_type: Vec<(usize, &str)> =
        find_template_variable_in_content(content, TemplateVariable::FilterAttributeType);
    let filter_attribute_name2: Vec<(usize, &str)> =
        find_template_variable_in_content(content, TemplateVariable::Filter2AttributeName);
    let filter_attribute_type2: Vec<(usize, &str)> =
        find_template_variable_in_content(content, TemplateVariable::Filter2AttributeType);

    match (filter_attribute_name.as_slice(), filter_attribute_type.as_slice(), filter_attribute_name2.as_slice(), filter_attribute_type2.as_slice()) {
        (
            [(attribute_name_idx, attribute_name)],
            [(attribute_type_idx, attribute_type)],
            [(attribute_name2_idx, _attribute_name2)],
            [(attribute_type2_idx, attribute_type2)]) => {
                let attribute_name_separator = content[attribute_name_idx + attribute_name.len()..*attribute_type_idx].to_string();
                let attributes_separator = content[attribute_type_idx + attribute_type.len()..*attribute_name2_idx].to_string();
                let attribute_name_naming_convention = detect_naming_convention(attribute_name);
                let attribute_type_naming_convention = detect_naming_convention(attribute_type);
                Ok(
                    content[..*attribute_name_idx].to_string() +
                    &filter.iter().map(|(attribute_name, attribute_type)| {
                    Ok(format!("{}{}{}", 
                    attribute_name_naming_convention.transform_string_to_naming_convention(attribute_name),
                    attribute_name_separator,
                    attribute_type_naming_convention.transform_string_to_naming_convention(&get_attribute_type(attribute_type, entities)?)))
                }).collect::<anyhow::Result<Vec<String>>>()?.join(&attributes_separator) +
                &content[*attribute_type2_idx + attribute_type2.len()..]
            )
            }
        (
            [(attribute_name_idx, attribute_name)],
            [],
            [(attribute_name2_idx, attribute_name2)],
            []
        ) => {
            let attribute_name_separator = content[attribute_name_idx + attribute_name.len()..*attribute_name2_idx].to_string();
            let attribute_name_naming_convention = detect_naming_convention(attribute_name);
            Ok(
                content[..*attribute_name_idx].to_string() +
                &filter.iter().map(|(attribute_name, _attribute_type)| {
                attribute_name_naming_convention.transform_string_to_naming_convention(attribute_name)
            }).collect::<Vec<String>>().join(&attribute_name_separator)
                + &content[*attribute_name2_idx + attribute_name2.len()..]
        )
        }
        _ => bail!("Invalid filter attribute content. Expected one of the following ways to define template attributes:TODO")
    }
}

fn get_attribute_type(
    attribute_type: &AttributeType,
    entities: &Vec<Entity>,
) -> anyhow::Result<String> {
    match attribute_type {
        AttributeType::Type(type_name) => Ok(type_name.clone()),
        AttributeType::ForeignKey(ForeignKey {
            entity_name,
            attribute_name,
        }) => {
            let foreign_key_entity = entities
                .iter()
                .find(|entity| entity.name == *entity_name)
                .expect(&format!(
                    "Foreign key entity {} is not present in the entities",
                    entity_name
                ));
            let foreign_key_attribute: &AttributeType = foreign_key_entity
                .attributes
                .get(attribute_name)
                .expect(&format!(
                    "Foreign key attribute {} is not present in the attributes of {}",
                    attribute_name, entity_name
                ));
            get_attribute_type(&foreign_key_attribute, entities)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::entity::{AttributeType, Entity, ForeignKey, Semantics},
        parsers::template_parser::TemplateParser,
    };

    #[test]
    fn test_entity_code_gen() {
        let string_test = std::fs::read_to_string("/Users/abdullahsabaaallil/Desktop/ene-lightspeed/template-rs/src/models/entity_name.rs").unwrap();
        let template_entity = TemplateParser::parse_template(&string_test).unwrap();
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
        let code = super::entity_codegen(&car, &entities, &template_entity).unwrap();
        std::fs::write("test.rs", code.clone())
            .map_err(|e| {
                println!("Error: {:?}", e);
                e
            })
            .unwrap();
        insta::assert_debug_snapshot!(code);
    }
}
 */