use pest::iterators::Pair;

use crate::codegen::composite_keys::{CompositeKey, CompositeKeyAttributeRep, CompositeKeyBody, CompositeKeyRep};
use anyhow::{anyhow, bail};
use super::template_parser::{Rule, TemplateParser};
use crate::parsers::template_parser::ERROR_MSG;

impl TemplateParser {

    fn parse_composite_key_attribute_rep(
        composite_key_attribute_rep_pair: Pair<Rule>,
    ) -> anyhow::Result<CompositeKeyAttributeRep> {
        match composite_key_attribute_rep_pair.as_rule() {
            Rule::composite_key_attribute_names_and_types_present => {
                let mut inner_pairs = composite_key_attribute_rep_pair.into_inner();
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
                Ok(CompositeKeyAttributeRep::NamesAndTypesPresent {
                    prefix,
                    template_attribute_name: attribute_name,
                    attribute_name_and_type_separator,
                    template_attribute_type: attribute_type,
                    attributes_separator,
                    suffix,
                })
            }
            Rule::composite_key_attribute_names_twice_present => {
                let mut inner_pairs = composite_key_attribute_rep_pair.into_inner();
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
                Ok(CompositeKeyAttributeRep::NamesTwicePresent {
                    prefix,
                    template_attribute_name: attribute_name,
                    attribute_name_twice_separator,
                    attributes_separator,
                    suffix,
                })
            }

            Rule::composite_key_attribute_names_present => {
                let mut inner_pairs = composite_key_attribute_rep_pair.into_inner();
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
                Ok(CompositeKeyAttributeRep::NamesPresent {
                    prefix,
                    template_attribute_name: attribute_name,
                    attributes_separator,
                    suffix,
                })
            }
            _ => bail!(
                "Unexpected rule: {:?} cannot be converted to an attribute repitition",
                composite_key_attribute_rep_pair
            ),
        }
    }

    pub fn parse_composite_key_rep(composite_key_rep_pair: Pair<Rule>) -> anyhow::Result<CompositeKeyRep> {
        match composite_key_rep_pair.as_rule() {
            Rule::consecutive_composite_keys => {
                let composite_key_pair = composite_key_rep_pair
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?;
                let composite_key = Self::parse_composite_key(composite_key_pair)?;
                Ok(CompositeKeyRep::ConsecutiveCompositeKeys { composite_key})
            }
            Rule::separated_composite_keys => {
                let mut inner_pairs = composite_key_rep_pair.into_inner();
                let composite_key1_pair = inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let composite_key = Self::parse_composite_key(composite_key1_pair)?;
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
                let composite_key2_pair = inner_pairs.next().ok_or(anyhow!("{ERROR_MSG}"))?;
                let _composite_key2 = Self::parse_composite_key(composite_key2_pair)?;
                Ok(CompositeKeyRep::SeparatedCompositeKeys { composite_key, separator })
            }
            _ => bail!(
                "Unexpected rule: {:?} cannot be converted to a composite key repetition",
                composite_key_rep_pair
            ),
        }
    }

    fn parse_composite_key(composite_key_pair: Pair<Rule>) -> anyhow::Result<CompositeKey> {
        let mut composite_key_bodies = Vec::new();
        for composite_key_body_pair in composite_key_pair.into_inner() {
            let composite_key_body = Self::parse_composite_key_body(
                composite_key_body_pair
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?,
            )?;
            composite_key_bodies.push(composite_key_body);
        }
        Ok(CompositeKey {
            content: composite_key_bodies,
        })
    }

    fn parse_composite_key_body(composite_key_body_pair: Pair<Rule>) -> anyhow::Result<CompositeKeyBody> {
        match composite_key_body_pair.as_rule() {
            Rule::content_with_msfa => {
                let content_with_msfa_pair = composite_key_body_pair
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?;
                let content_with_msfa = Self::parse_content_with_msfa(content_with_msfa_pair)?;
                Ok(CompositeKeyBody::ContentWithMSFA(content_with_msfa))
            }
            Rule::composite_key_attribute_rep => {
                let composite_key_attribute_rep_pair = composite_key_body_pair
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?;
                let composite_key_attribute_rep =
                    Self::parse_composite_key_attribute_rep(composite_key_attribute_rep_pair)?;
                Ok(CompositeKeyBody::CompositeKeyAttributeRep(composite_key_attribute_rep))
            }
            _ => bail!(
                "Unexpected rule: {:?} cannot be converted to a composite key body",
                composite_key_body_pair
            ),
        }
    }
}