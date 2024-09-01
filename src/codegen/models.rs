use crate::models::entity::Entity;

use super::{contents::EntityTemplateContent, CodeGen};
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