use anyhow::bail;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./parsers/template_grammar.pest"]
pub struct TemplateParser;

impl TemplateParser {
    pub fn parse_template(template: &str) -> anyhow::Result<TemplateEntity> {
        let pairs = TemplateParser::parse(Rule::entity, template)?
            .next()
            .unwrap();
        let entity_content = pairs
            .into_inner()
            .map(TemplateParser::inner_parse_rule)
            .collect::<anyhow::Result<Vec<TemplateEntityContent>>>()?;
        Ok(TemplateEntity { entity_content })
    }

    fn inner_parse_rule(pair: Pair<Rule>) -> anyhow::Result<TemplateEntityContent> {
        match pair.as_rule() {
            Rule::content => Ok(TemplateEntityContent::Static {
                content: pair.as_str().to_string(),
                start: pair.as_span().start(),
                end: pair.as_span().end(),
            }),
            Rule::attribute => Ok(TemplateEntityContent::Attribute {
                content: pair
                    .clone()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string(),
                start: pair.as_span().start(),
                end: pair.as_span().end(),
            }),
            Rule::filter => Ok(TemplateEntityContent::Filter {
                content: pair
                    .clone()
                    .into_inner()
                    .map(TemplateParser::inner_parse_filter)
                    .collect::<anyhow::Result<Vec<FilterContent>>>()?,
            }),
            Rule::entity_content => {
                TemplateParser::inner_parse_rule(pair.into_inner().next().unwrap())
            }
            s => bail!("Unexpected rule: {:?}", s),
        }
    }

    fn inner_parse_filter(pair: Pair<Rule>) -> anyhow::Result<FilterContent> {
        match pair.as_rule() {
            Rule::content => Ok(FilterContent::Static {
                content: pair.as_str().to_string(),
                start: pair.as_span().start(),
                end: pair.as_span().end(),
            }),
            Rule::filter_attribute => Ok(FilterContent::FilterAttribute {
                content: pair
                    .clone()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .to_string(),
                start: pair.as_span().start(),
                end: pair.as_span().end(),
            }),
            Rule::filter_content => {
                TemplateParser::inner_parse_filter(pair.into_inner().next().unwrap())
            }
            s => bail!("Unexpected rule: {:?}", s),
        }
    }
}

#[derive(Debug)]
pub struct TemplateEntity {
    pub entity_content: Vec<TemplateEntityContent>,
}

#[derive(Debug)]
pub enum TemplateEntityContent {
    Static {
        content: String,
        start: usize,
        end: usize,
    },
    Attribute {
        content: String,
        start: usize,
        end: usize,
    },
    Filter {
        content: Vec<FilterContent>,
    },
}

#[derive(Debug)]
pub enum FilterContent {
    Static {
        content: String,
        start: usize,
        end: usize,
    },
    FilterAttribute {
        content: String,
        start: usize,
        end: usize,
    },
}

#[cfg(test)]
mod tests {
    use crate::parsers::template_parser::TemplateParser;

    #[test]
    fn test_parser() {
        let string_test = std::fs::read_to_string("/Users/abdullahsabaaallil/Desktop/ene-lightspeed/template-rs/src/models/entity_name.rs").unwrap();
        let template_entity = TemplateParser::parse_template(&string_test);
        insta::assert_debug_snapshot!(template_entity);
    }
}
