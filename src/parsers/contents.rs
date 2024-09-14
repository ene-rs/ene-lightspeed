use pest::iterators::Pair;

use crate::codegen::contents::{Content, ContentWithMSA};

use super::template_parser::{Rule, TemplateParser};
use anyhow::{anyhow, bail};
use crate::parsers::template_parser::ERROR_MSG;

impl TemplateParser {
    pub fn parse_content_with_msa(
        content_with_msa_pair: Pair<Rule>,
    ) -> anyhow::Result<ContentWithMSA> {
        match content_with_msa_pair.as_rule() {
            Rule::content => {
                let content_pair = content_with_msa_pair.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?;
                Ok(ContentWithMSA::Content(Self::parse_content(content_pair)?))
            }
            Rule::most_specific_attribute_name => {
                let most_specific_attribute_filter_name_pair = content_with_msa_pair.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?;
                Ok(ContentWithMSA::MostSpecificFilterAttributeName(Self::parse_string_with_naming_convention(&most_specific_attribute_filter_name_pair)?))
            }
            Rule::most_specific_attribute_type => {
                let most_specific_attribute_filter_type_pair = content_with_msa_pair.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?;
                Ok(ContentWithMSA::MostSpecificFilterAttributeType(Self::parse_string_with_naming_convention(&most_specific_attribute_filter_type_pair)?))
            }
            _ => bail!("Unexpected rule: {:?} cannot be converted to a content with most specific filter attribute", content_with_msa_pair)
        }
    }



    pub fn parse_content(content_pair: Pair<Rule>) -> anyhow::Result<Content> {
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
}