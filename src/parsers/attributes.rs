use pest::iterators::Pair;

use crate::codegen::attributes::AttributeRep;

use super::template_parser::{Rule, TemplateParser};
use anyhow::{anyhow, bail};
use crate::parsers::template_parser::ERROR_MSG;


impl TemplateParser {
    pub fn parse_attribute_rep(attribute_pair: Pair<Rule>) -> anyhow::Result<AttributeRep> {
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
}