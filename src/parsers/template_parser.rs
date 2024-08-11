use anyhow::{anyhow, bail};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::codegen::models::{
    AttributeRep, Content, ContentWithMSFA, EntityTemplate, EntityTemplateContent, Filter,
    FilterAttributeRep, FilterBody, FilterRep, StringWithNamingConvention,
};

static ERROR_MSG: &str = "Could not parse the template. This is likely a bug in the code.\n A report is very much appreciated!";

#[derive(Parser)]
#[grammar = "./parsers/template_grammar.pest"]
pub struct TemplateParser;

impl TemplateParser {
    pub fn parse_template(template: &str) -> anyhow::Result<EntityTemplate> {
        let mut pairs: Pairs<Rule> = TemplateParser::parse(Rule::entity_rep, template)?;
        let mut entity_content = Vec::new();
        for pair in pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?.into_inner() {
            match pair.as_rule() {
                Rule::entity_content => {
                    let entity_content_pair =
                        pair.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?;
                    match entity_content_pair.as_rule() {
                        Rule::content => {
                            let entity_content_pair = entity_content_pair.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?;
                            entity_content.push(EntityTemplateContent::Content(Self::parse_content(entity_content_pair)?));
                        }
                        Rule::attribute_rep => {
                            let attribute_rep_pair = entity_content_pair.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?;
                            entity_content.push(EntityTemplateContent::AttributeRep(Self::parse_attribute_rep(attribute_rep_pair)?));
                        }
                        Rule::filter_rep => {
                            let filter_rep_pair = entity_content_pair.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?;
                            entity_content.push(EntityTemplateContent::FilterRep(Self::parse_filter_rep(filter_rep_pair)?));
                        }
                        _ => bail!("Unexpected rule: {:?} cannot be converted to an entity template content", entity_content_pair)
                    }
                }
                _ => bail!(
                    "Unexpected rule: {:?} cannot be converted to an entity content",
                    pair
                ),
            }
        }
        Ok(EntityTemplate { entity_content })
    }

    fn parse_filter_attribute_rep(
        filter_attribute_rep_pair: Pair<Rule>,
    ) -> anyhow::Result<FilterAttributeRep> {
        match filter_attribute_rep_pair.as_rule() {
            Rule::filter_attribute_names_and_types_present => {
                let mut inner_pairs = filter_attribute_rep_pair.into_inner();
                let prefix = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_name = Self::parse_string_with_naming_convention(
                    &inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_name_and_type_separator = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_type = Self::parse_string_with_naming_convention(
                    &inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attributes_separator = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let _attribute2_name = &inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let _attribute2_name_separator =
                    inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let _attribute2_type = inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let suffix = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                Ok(FilterAttributeRep::NamesAndTypesPresent {
                    prefix,
                    template_attribute_name: attribute_name,
                    attribute_name_and_type_separator,
                    template_attribute_type: attribute_type,
                    attributes_separator,
                    suffix,
                })
            }
            Rule::filter_attribute_names_twice_present => {
                let mut inner_pairs = filter_attribute_rep_pair.into_inner();
                let prefix = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_name = Self::parse_string_with_naming_convention(
                    &inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_name_twice_separator = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let _attribute_name = &inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let attributes_separator = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let _attribute2_name = inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let _attribute2_name_separator =
                    inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let _attribute2_name = inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let suffix = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                Ok(FilterAttributeRep::NamesTwicePresent {
                    prefix,
                    template_attribute_name: attribute_name,
                    attribute_name_twice_separator,
                    attributes_separator,
                    suffix,
                })
            }

            Rule::filter_attribute_names_present => {
                let mut inner_pairs = filter_attribute_rep_pair.into_inner();
                let prefix = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_name = Self::parse_string_with_naming_convention(
                    &inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attributes_separator = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let _attribute2_name = inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let suffix = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                Ok(FilterAttributeRep::NamesPresent {
                    prefix,
                    template_attribute_name: attribute_name,
                    attributes_separator,
                    suffix,
                })
            }
            _ => bail!(
                "Unexpected rule: {:?} cannot be converted to an attribute repitition",
                filter_attribute_rep_pair
            ),
        }
    }
    fn parse_filter_rep(filter_rep_pair: Pair<Rule>) -> anyhow::Result<FilterRep> {
        match filter_rep_pair.as_rule() {
            Rule::consecutive_filters => {
                let filter_pair = filter_rep_pair
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?;
                let filter = Self::parse_filter(filter_pair)?;
                Ok(FilterRep::ConsecutiveFilters { filter })
            }
            Rule::separated_filters => {
                let mut inner_pairs = filter_rep_pair.into_inner();
                let filter1_pair = inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let filter = Self::parse_filter(filter1_pair)?;
                let separator = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let filter2_pair = inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let _filter2 = Self::parse_filter(filter2_pair)?;
                Ok(FilterRep::SeparatedFilters { filter, separator })
            }
            _ => bail!(
                "Unexpected rule: {:?} cannot be converted to a filter repetition",
                filter_rep_pair
            ),
        }
    }

    fn parse_filter(filter_pair: Pair<Rule>) -> anyhow::Result<Filter> {
        let mut filter_bodies = Vec::new();
        for filter_body_pair in filter_pair.into_inner() {
            let filter_body = Self::parse_filter_body(
                filter_body_pair
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?,
            )?;
            filter_bodies.push(filter_body);
        }
        Ok(Filter {
            content: filter_bodies,
        })
    }

    fn parse_filter_body(filter_body_pair: Pair<Rule>) -> anyhow::Result<FilterBody> {
        match filter_body_pair.as_rule() {
            Rule::content_with_msfa => {
                let content_with_msfa_pair = filter_body_pair
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?;
                let content_with_msfa = Self::parse_content_with_msfa(content_with_msfa_pair)?;
                Ok(FilterBody::ContentWithMSFA(content_with_msfa))
            }
            Rule::filter_attribute_rep => {
                let filter_attribute_rep_pair = filter_body_pair
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?;
                let filter_attribute_rep =
                    Self::parse_filter_attribute_rep(filter_attribute_rep_pair)?;
                Ok(FilterBody::FilterAttributeRep(filter_attribute_rep))
            }
            _ => bail!(
                "Unexpected rule: {:?} cannot be converted to a filter body",
                filter_body_pair
            ),
        }
    }

    fn parse_content_with_msfa(
        content_with_msfa_pair: Pair<Rule>,
    ) -> anyhow::Result<ContentWithMSFA> {
        match content_with_msfa_pair.as_rule() {
            Rule::content => {
                let content_pair = content_with_msfa_pair.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?;
                Ok(ContentWithMSFA::Content(Self::parse_content(content_pair)?))
            }
            Rule::most_specific_attribute_filter_name => {
                let most_specific_attribute_filter_name_pair = content_with_msfa_pair.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?;
                Ok(ContentWithMSFA::MostSpecificFilterAttributeName(Self::parse_string_with_naming_convention(&most_specific_attribute_filter_name_pair)?))
            }
            Rule::most_specific_attribute_filter_type => {
                let most_specific_attribute_filter_type_pair = content_with_msfa_pair.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?;
                Ok(ContentWithMSFA::MostSpecificFilterAttributeType(Self::parse_string_with_naming_convention(&most_specific_attribute_filter_type_pair)?))
            }
            _ => bail!("Unexpected rule: {:?} cannot be converted to a content with most specific filter attribute", content_with_msfa_pair)
        }
    }

    fn parse_attribute_rep(attribute_pair: Pair<Rule>) -> anyhow::Result<AttributeRep> {
        match attribute_pair.as_rule() {
            Rule::attribute_names_and_types_present => {
                let mut inner_pairs = attribute_pair.into_inner();
                let prefix = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_name = Self::parse_string_with_naming_convention(
                    &inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_name_and_type_separator = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_type = Self::parse_string_with_naming_convention(
                    &inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attributes_separator = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let _attribute2_name = &inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let _attribute2_name_separator =
                    inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let _attribute2_type = inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let suffix = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                Ok(AttributeRep::NamesAndTypesPresent {
                    prefix,
                    template_attribute_name: attribute_name,
                    attribute_name_and_type_separator,
                    template_attribute_type: attribute_type,
                    attributes_separator,
                    suffix,
                })
            }
            Rule::attribute_names_twice_present => {
                let mut inner_pairs = attribute_pair.into_inner();
                let prefix = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_name = Self::parse_string_with_naming_convention(
                    &inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_name_twice_separator = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let _attribute_name = &inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let attributes_separator = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let _attribute2_name = inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let _attribute2_name_separator =
                    inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let _attribute2_name = inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let suffix = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                Ok(AttributeRep::NamesTwicePresent {
                    prefix,
                    template_attribute_name: attribute_name,
                    attribute_name_twice_separator,
                    attributes_separator,
                    suffix,
                })
            }

            Rule::attribute_names_present => {
                let mut inner_pairs = attribute_pair.into_inner();
                let prefix = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_name = Self::parse_string_with_naming_convention(
                    &inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attributes_separator =
                    Self::parse_content(inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?)?;
                let _attribute2_name = inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let suffix = Self::parse_content(
                    inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                Ok(AttributeRep::NamesPresent {
                    prefix,
                    template_attribute_name: attribute_name,
                    attributes_separator,
                    suffix,
                })
            }
            _ => bail!(
                "Unexpected rule: {:?} cannot be converted to an attribute repitition",
                attribute_pair
            ),
        }
    }

    fn parse_content(content_pair: Pair<Rule>) -> anyhow::Result<Content> {
        match content_pair.as_rule() {
            Rule::static_content => Ok(Content::Static(content_pair.clone().as_str().to_string())),
            Rule::entity_name => {
                let inner_entity_name = content_pair
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?;
                Ok(Content::EntityName(
                    Self::parse_string_with_naming_convention(&inner_entity_name)?,
                ))
            }
            Rule::entity_plural => {
                let inner_entity_plural = content_pair
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?;
                Ok(Content::EntityPlural(
                    Self::parse_string_with_naming_convention(&inner_entity_plural)?,
                ))
            }
            Rule::primary_key => {
                let inner_primary_key = content_pair
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?;
                Ok(Content::PrimaryKey(
                    Self::parse_string_with_naming_convention(&inner_primary_key)?,
                ))
            }
            _ => bail!(
                "Unexpected rule: {:?} cannot be converted to a content",
                content_pair
            ),
        }
    }

    fn parse_string_with_naming_convention(
        pair: &Pair<Rule>,
    ) -> anyhow::Result<StringWithNamingConvention> {
        match pair.as_rule() {
            Rule::attribute_name_kebab_case
            | Rule::attribute_type_kebab_case
            | Rule::entity_name_kebab_case
            | Rule::entity_plural_kebab_case
            | Rule::primary_key_kebab_case
            | Rule::most_specific_attribute_filter_name_kebab_case
            | Rule::most_specific_attribute_filter_type_kebab_case
            | Rule::filter_attribute_name_kebab_case
            | Rule::filter_attribute_type_kebab_case => Ok(StringWithNamingConvention::KebabCase),

            Rule::attribute_name_lower_camel_case
            | Rule::attribute_type_lower_camel_case
            | Rule::entity_name_lower_camel_case
            | Rule::entity_plural_lower_camel_case
            | Rule::primary_key_lower_camel_case
            | Rule::most_specific_attribute_filter_name_lower_camel_case
            | Rule::most_specific_attribute_filter_type_lower_camel_case
            | Rule::filter_attribute_name_lower_camel_case
            | Rule::filter_attribute_type_lower_camel_case => {
                Ok(StringWithNamingConvention::LowerCamelCase)
            }

            Rule::attribute_name_upper_camel_case
            | Rule::attribute_type_upper_camel_case
            | Rule::entity_name_upper_camel_case
            | Rule::entity_plural_upper_camel_case
            | Rule::primary_key_upper_camel_case
            | Rule::most_specific_attribute_filter_name_upper_camel_case
            | Rule::most_specific_attribute_filter_type_upper_camel_case
            | Rule::filter_attribute_name_upper_camel_case
            | Rule::filter_attribute_type_upper_camel_case => {
                Ok(StringWithNamingConvention::UpperCamelCase)
            }

            Rule::attribute_name_shouty_kebab_case
            | Rule::attribute_type_shouty_kebab_case
            | Rule::entity_name_shouty_kebab_case
            | Rule::entity_plural_shouty_kebab_case
            | Rule::primary_key_shouty_kebab_case
            | Rule::most_specific_attribute_filter_name_shouty_kebab_case
            | Rule::most_specific_attribute_filter_type_shouty_kebab_case
            | Rule::filter_attribute_name_shouty_kebab_case
            | Rule::filter_attribute_type_shouty_kebab_case => {
                Ok(StringWithNamingConvention::ShoutyKebabCase)
            }

            Rule::attribute_name_shouty_snake_case
            | Rule::attribute_type_shouty_snake_case
            | Rule::entity_name_shouty_snake_case
            | Rule::entity_plural_shouty_snake_case
            | Rule::primary_key_shouty_snake_case
            | Rule::most_specific_attribute_filter_name_shouty_snake_case
            | Rule::most_specific_attribute_filter_type_shouty_snake_case
            | Rule::filter_attribute_name_shouty_snake_case
            | Rule::filter_attribute_type_shouty_snake_case => {
                Ok(StringWithNamingConvention::ShoutySnakeCase)
            }

            Rule::attribute_name_snake_case
            | Rule::attribute_type_snake_case
            | Rule::entity_name_snake_case
            | Rule::entity_plural_snake_case
            | Rule::primary_key_snake_case
            | Rule::most_specific_attribute_filter_name_snake_case
            | Rule::most_specific_attribute_filter_type_snake_case
            | Rule::filter_attribute_name_snake_case
            | Rule::filter_attribute_type_snake_case => Ok(StringWithNamingConvention::SnakeCase),

            Rule::attribute_name_title_case
            | Rule::attribute_type_title_case
            | Rule::entity_name_title_case
            | Rule::entity_plural_title_case
            | Rule::primary_key_title_case
            | Rule::most_specific_attribute_filter_name_title_case
            | Rule::most_specific_attribute_filter_type_title_case
            | Rule::filter_attribute_name_title_case
            | Rule::filter_attribute_type_title_case => Ok(StringWithNamingConvention::TitleCase),

            Rule::attribute_name_train_case
            | Rule::attribute_type_train_case
            | Rule::entity_name_train_case
            | Rule::entity_plural_train_case
            | Rule::primary_key_train_case
            | Rule::most_specific_attribute_filter_name_train_case
            | Rule::most_specific_attribute_filter_type_train_case
            | Rule::filter_attribute_name_train_case
            | Rule::filter_attribute_type_train_case => Ok(StringWithNamingConvention::TrainCase),

            _ => bail!(
                "Unexpected rule: {:?} cannot be converted to a string with naming convention",
                pair
            ),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn experiment() {
        let template = std::fs::read_to_string("/Users/abdullahsabaaallil/Desktop/ene-lightspeed/template-rs/src/models/entity_name.rs").unwrap();
        let parsed: crate::codegen::models::EntityTemplate =
            super::TemplateParser::parse_template(&template).unwrap();
        println!("{:?}", parsed);
    }
}
