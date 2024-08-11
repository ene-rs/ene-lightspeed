use std::{collections::BTreeMap, fmt::Display};

use anyhow::bail;
use serde::{Deserialize, Serialize};

pub type AttributeName = String;
pub type FilterByAttributeNames = Vec<AttributeName>;

pub type FilterByAttributes = Vec<(AttributeName, AttributeType)>;

pub trait MostSpecificFilterAttributes {
    fn get_most_specific_filter_attribute(&self) -> (AttributeName, AttributeType);
}

impl MostSpecificFilterAttributes for &FilterByAttributes {
    fn get_most_specific_filter_attribute(&self) -> (AttributeName, AttributeType) {
        self.last().unwrap().clone()
    }
}
pub type UniqueAttributes = Vec<AttributeName>;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum AttributeType {
    Type(String),
    ForeignKey(ForeignKey),
}

impl AttributeType {
    pub fn get_attribute_type(&self, entities: &Vec<Entity>) -> anyhow::Result<String> {
        match self {
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
                foreign_key_attribute.get_attribute_type(entities)
            }
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Entity {
    pub name: String,
    pub attributes: BTreeMap<AttributeName, AttributeType>,
    pub primary_key: AttributeName,
    pub filter_by: Vec<FilterByAttributeNames>,
    pub unique_attributes: Vec<UniqueAttributes>,
    pub semantics: Semantics,
}

impl Entity {
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
    pub fn validate_request(&self, entities: Vec<Entity>) -> anyhow::Result<()> {
        let attribute_names = self.attributes.keys().collect::<Vec<&String>>();
        if !attribute_names.contains(&&self.primary_key) {
            bail!(
                "Primary key {} is not present in the attributes of {}",
                self.primary_key,
                self.name
            );
        }

        for foreign_key in self.get_foreign_keys().iter() {
            // search the foreign key in the other entities
            let foreign_key_entity = entities
                .iter()
                .find(|entity| entity.name == foreign_key.entity_name)
                .expect(&format!(
                    "Foreign key entity {} is not present in the entities",
                    foreign_key.entity_name
                ));
            if !foreign_key_entity
                .attributes
                .iter()
                .map(|(attribute_name, _)| attribute_name)
                .collect::<Vec<&String>>()
                .contains(&&foreign_key.attribute_name)
            {
                bail!(
                    "Foreign key attribute {} is not present in the attributes of {}",
                    foreign_key.attribute_name,
                    foreign_key.entity_name
                );
            }
        }

        for unique_attribute in self
            .unique_attributes
            .iter()
            .flatten()
            .collect::<Vec<&String>>()
        {
            if self.attributes.get(unique_attribute).is_none() {
                bail!(
                    "Unique attribute {} is not present in the attributes of {}",
                    unique_attribute,
                    self.name
                );
            }
        }
        for filter_by_attribute in self.filter_by.iter().flatten().collect::<Vec<&String>>() {
            if self.attributes.get(filter_by_attribute).is_none() {
                bail!(
                    "Filter by attribute {} is not present in the attributes of {}",
                    filter_by_attribute,
                    self.name
                );
            }
        }

        // Verify that the unique attributes are present in the filter_by
        for unique_attribute in self.unique_attributes.iter() {
            if !self.filter_by.contains(unique_attribute) {
                bail!(
                    "Unique attribute {:?} is not present in the filter_by of {}",
                    unique_attribute,
                    self.name
                );
            }

            let common = self
                .filter_by
                .iter()
                .filter(|filter_by| filter_by != &unique_attribute)
                .any(|filter_by| {
                    println!("{:?}", filter_by);
                    is_sub(&filter_by, &unique_attribute)
                });

            if common {
                bail!("Unique attribute {:?} is a sub attribute of another filter_by in {}. It does not make sense to have fine grained filters on unique attributes, since they're unique", unique_attribute, self.name);
            }
        }

        Ok(())
    }

    pub fn get_foreign_keys(&self) -> Vec<ForeignKey> {
        let mut foreign_keys = Vec::new();
        for (_, attribute_type) in &self.attributes {
            match attribute_type {
                AttributeType::ForeignKey(foreign_key) => {
                    foreign_keys.push(foreign_key.clone());
                }
                _ => {}
            }
        }
        foreign_keys
    }

    pub fn get_filter_by_attributes(&self) -> Vec<FilterByAttributes> {
        let mut filter_by_attributes = Vec::new();
        for filter_by in &self.filter_by {
            let mut filter_by_attribute = Vec::new();
            for attribute_name in filter_by {
                filter_by_attribute.push((
                    attribute_name.clone(),
                    self.attributes.get(attribute_name).unwrap().clone(),
                ));
            }
            filter_by_attributes.push(filter_by_attribute);
        }
        filter_by_attributes
    }
}

fn is_sub<T: PartialEq>(mut haystack: &[T], needle: &[T]) -> bool {
    if needle.len() == 0 {
        return true;
    }
    while !haystack.is_empty() {
        if haystack.starts_with(needle) {
            return true;
        }
        haystack = &haystack[1..];
    }
    false
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Semantics {
    pub plural: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ForeignKey {
    pub entity_name: String,
    pub attribute_name: String,
}

impl Display for AttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
