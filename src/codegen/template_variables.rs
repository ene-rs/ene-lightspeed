use std::{fmt::Display, str::FromStr};

use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToTrainCase, ToUpperCamelCase,
};
use strum::EnumIter;

#[derive(Debug, Clone, EnumIter)]
pub enum TemplateVariable {
    AttributeName,
    AttributeType,
    Attribute2Name,
    Attribute2Type,
    EntityName,
    EntityNamePlural,
    PrimaryKey,
    MostSpecificFilterAttributeName,
    MostSpecificFilterAttributeType,
    FilterAttributeName,
    FilterAttributeType,
    Filter2AttributeName,
    Filter2AttributeType,
}

impl FromStr for TemplateVariable {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_snake_case().as_str() {
            "attribute_name" => Ok(TemplateVariable::AttributeName),
            "attribute_type" => Ok(TemplateVariable::AttributeType),
            "attribute2_name" => Ok(TemplateVariable::Attribute2Name),
            "attribute2_type" => Ok(TemplateVariable::Attribute2Type),
            "entity_name" => Ok(TemplateVariable::EntityName),
            "entity_name_plural" => Ok(TemplateVariable::EntityNamePlural),
            "primary_key" => Ok(TemplateVariable::PrimaryKey),
            "most_specific_filter_attribute_name" => {
                Ok(TemplateVariable::MostSpecificFilterAttributeName)
            }
            "most_specific_filter_attribute_type" => {
                Ok(TemplateVariable::MostSpecificFilterAttributeType)
            }
            "filter_attribute_name" => Ok(TemplateVariable::FilterAttributeName),
            "filter_attribute_type" => Ok(TemplateVariable::FilterAttributeType),
            "filter2_attribute_name" => Ok(TemplateVariable::Filter2AttributeName),
            "filter2_attribute_type" => Ok(TemplateVariable::Filter2AttributeType),
            _ => Err(anyhow::anyhow!("Could not parse the template variable")),
        }
    }
}

impl Display for TemplateVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn find_template_variable_in_content(
    content: &str,
    template_variable: TemplateVariable,
) -> Vec<(usize, &str)> {
    let mut res = Vec::new();
    res.extend(content.match_indices(&template_variable.to_string().to_kebab_case()));
    res.extend(content.match_indices(&template_variable.to_string().to_lower_camel_case()));
    res.extend(content.match_indices(&template_variable.to_string().to_upper_camel_case()));
    res.extend(content.match_indices(&template_variable.to_string().to_shouty_kebab_case()));
    res.extend(content.match_indices(&template_variable.to_string().to_shouty_snake_case()));
    res.extend(content.match_indices(&template_variable.to_string().to_snake_case()));
    res.extend(content.match_indices(&template_variable.to_string().to_title_case()));
    res.extend(content.match_indices(&template_variable.to_string().to_train_case()));

    res
}

#[cfg(test)]
mod tests {
    use super::{find_template_variable_in_content, TemplateVariable};

    #[test]
    fn test_upper_camel_case() {
        let with_entity_name = "here there is EntityName";
        let entity_name_found =
            find_template_variable_in_content(with_entity_name, TemplateVariable::EntityName);
        assert!(entity_name_found.len() == 1);
    }
}
