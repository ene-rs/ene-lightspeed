use anyhow::bail;
use pest::iterators::Pair;

use crate::codegen::models::StringWithNamingConvention;

use super::template_parser::{Rule, TemplateParser};

impl TemplateParser {

    pub fn parse_string_with_naming_convention(
        pair: &Pair<Rule>,
    ) -> anyhow::Result<StringWithNamingConvention> {
        match pair.as_rule() {
            Rule::attribute_name_kebab_case
            | Rule::attribute_type_kebab_case
            | Rule::entity_name_kebab_case
            | Rule::entity_plural_kebab_case
            | Rule::primary_key_kebab_case
            | Rule::most_specific_attribute_name_kebab_case
            | Rule::most_specific_attribute_type_kebab_case
            | Rule::filter_attribute_name_kebab_case
            | Rule::filter_attribute_type_kebab_case 
            | Rule::composite_key_attribute_name_kebab_case
            | Rule::composite2_key_attribute_name_kebab_case
            | Rule::composite_key_attribute_type_kebab_case
            | Rule::composite2_key_attribute_type_kebab_case
            | Rule::filter2_attribute_name_kebab_case
            | Rule::filter2_attribute_type_kebab_case => Ok(StringWithNamingConvention::KebabCase),

            Rule::attribute_name_lower_camel_case
            | Rule::attribute_type_lower_camel_case
            | Rule::entity_name_lower_camel_case
            | Rule::entity_plural_lower_camel_case
            | Rule::primary_key_lower_camel_case
            | Rule::most_specific_attribute_name_lower_camel_case
            | Rule::most_specific_attribute_type_lower_camel_case
            | Rule::filter_attribute_name_lower_camel_case
            | Rule::filter_attribute_type_lower_camel_case 
            | Rule::composite_key_attribute_name_lower_camel_case
            | Rule::composite2_key_attribute_name_lower_camel_case
            | Rule::composite_key_attribute_type_lower_camel_case
            | Rule::composite2_key_attribute_type_lower_camel_case
            | Rule::filter2_attribute_name_lower_camel_case
            | Rule::filter2_attribute_type_lower_camel_case => Ok(StringWithNamingConvention::LowerCamelCase),
            

            Rule::attribute_name_upper_camel_case
            | Rule::attribute_type_upper_camel_case
            | Rule::entity_name_upper_camel_case
            | Rule::entity_plural_upper_camel_case
            | Rule::primary_key_upper_camel_case
            | Rule::most_specific_attribute_name_upper_camel_case
            | Rule::most_specific_attribute_type_upper_camel_case
            | Rule::filter_attribute_name_upper_camel_case
            | Rule::filter_attribute_type_upper_camel_case 
            | Rule::composite_key_attribute_name_upper_camel_case
            | Rule::composite2_key_attribute_name_upper_camel_case
            | Rule::composite_key_attribute_type_upper_camel_case
            | Rule::composite2_key_attribute_type_upper_camel_case
            | Rule::filter2_attribute_name_upper_camel_case
            | Rule::filter2_attribute_type_upper_camel_case => Ok(StringWithNamingConvention::UpperCamelCase),
            

            Rule::attribute_name_shouty_kebab_case
            | Rule::attribute_type_shouty_kebab_case
            | Rule::entity_name_shouty_kebab_case
            | Rule::entity_plural_shouty_kebab_case
            | Rule::primary_key_shouty_kebab_case
            | Rule::most_specific_attribute_name_shouty_kebab_case
            | Rule::most_specific_attribute_type_shouty_kebab_case
            | Rule::filter_attribute_name_shouty_kebab_case
            | Rule::filter_attribute_type_shouty_kebab_case
            | Rule::composite_key_attribute_name_shouty_kebab_case
            | Rule::composite2_key_attribute_name_shouty_kebab_case
            | Rule::composite_key_attribute_type_shouty_kebab_case
            | Rule::composite2_key_attribute_type_shouty_kebab_case
            | Rule::filter2_attribute_name_shouty_kebab_case
            | Rule::filter2_attribute_type_shouty_kebab_case
             => Ok(StringWithNamingConvention::ShoutyKebabCase),
            

            Rule::attribute_name_shouty_snake_case
            | Rule::attribute_type_shouty_snake_case
            | Rule::entity_name_shouty_snake_case
            | Rule::entity_plural_shouty_snake_case
            | Rule::primary_key_shouty_snake_case
            | Rule::most_specific_attribute_name_shouty_snake_case
            | Rule::most_specific_attribute_type_shouty_snake_case
            | Rule::filter_attribute_name_shouty_snake_case
            | Rule::filter_attribute_type_shouty_snake_case
            | Rule::composite_key_attribute_name_shouty_snake_case
            | Rule::composite2_key_attribute_name_shouty_snake_case
            | Rule::composite_key_attribute_type_shouty_snake_case
            | Rule::composite2_key_attribute_type_shouty_snake_case
            | Rule::filter2_attribute_name_shouty_snake_case
            | Rule::filter2_attribute_type_shouty_snake_case => Ok(StringWithNamingConvention::ShoutySnakeCase),
            

            Rule::attribute_name_snake_case
            | Rule::attribute_type_snake_case
            | Rule::entity_name_snake_case
            | Rule::entity_plural_snake_case
            | Rule::primary_key_snake_case
            | Rule::most_specific_attribute_name_snake_case
            | Rule::most_specific_attribute_type_snake_case
            | Rule::filter_attribute_name_snake_case
            | Rule::filter_attribute_type_snake_case
            | Rule::composite_key_attribute_name_snake_case
            | Rule::composite2_key_attribute_name_snake_case
            | Rule::composite_key_attribute_type_snake_case
            | Rule::composite2_key_attribute_type_snake_case
            | Rule::filter2_attribute_name_snake_case
            | Rule::filter2_attribute_type_snake_case => Ok(StringWithNamingConvention::SnakeCase),

            Rule::attribute_name_title_case
            | Rule::attribute_type_title_case
            | Rule::entity_name_title_case
            | Rule::entity_plural_title_case
            | Rule::primary_key_title_case
            | Rule::most_specific_attribute_name_title_case
            | Rule::most_specific_attribute_type_title_case
            | Rule::filter_attribute_name_title_case
            | Rule::filter_attribute_type_title_case
            | Rule::composite_key_attribute_name_title_case
            | Rule::composite_key_attribute_type_title_case
            | Rule::composite2_key_attribute_type_title_case
            | Rule::composite2_key_attribute_name_title_case 
            | Rule::filter2_attribute_name_title_case
            | Rule::filter2_attribute_type_title_case => Ok(StringWithNamingConvention::TitleCase),

            Rule::attribute_name_train_case
            | Rule::attribute_type_train_case
            | Rule::entity_name_train_case
            | Rule::entity_plural_train_case
            | Rule::primary_key_train_case
            | Rule::most_specific_attribute_name_train_case
            | Rule::most_specific_attribute_type_train_case
            | Rule::filter_attribute_name_train_case
            | Rule::filter_attribute_type_train_case
            | Rule::composite_key_attribute_name_train_case
            | Rule::composite2_key_attribute_name_train_case
            | Rule::composite_key_attribute_type_train_case
            | Rule::composite2_key_attribute_type_train_case
            | Rule::filter2_attribute_name_train_case
            | Rule::filter2_attribute_type_train_case => Ok(StringWithNamingConvention::TrainCase),

            _ => bail!(
                "Unexpected rule: {:?} cannot be converted to a string with naming convention",
                pair
            ),
        }
    }
}