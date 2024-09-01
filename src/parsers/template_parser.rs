use anyhow::{anyhow, bail};
use pest::{
    iterators::Pairs,
    Parser,
};
use pest_derive::Parser;

use crate::codegen::{contents::EntityTemplateContent, models::EntityTemplate, };

pub static ERROR_MSG: &str = "Could not parse the template. This is likely a bug in the code.\n A report is very much appreciated!";

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
                        Rule::composite_key_rep => {
                            let composite_key_rep_pair = entity_content_pair.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?.into_inner().next().ok_or(anyhow!("{ERROR_MSG}"))?;
                            entity_content.push(EntityTemplateContent::CompositeKeyRep(Self::parse_composite_key_rep(composite_key_rep_pair)?));
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
