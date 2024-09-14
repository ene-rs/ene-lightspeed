use pest::iterators::Pair;

use crate::codegen::filters::{Filter, FilterAttributeRep, FilterBody, FilterRep};

use super::template_parser::{Rule, TemplateParser};
use anyhow::{anyhow, bail};
use crate::parsers::template_parser::ERROR_MSG;

impl TemplateParser {
    fn parse_filter_attribute_rep(
        filter_attribute_rep_pair: Pair<Rule>,
    ) -> anyhow::Result<FilterAttributeRep> {
        match filter_attribute_rep_pair.as_rule() {
            Rule::filter_attribute_names_and_types_present => {
                let mut inner_pairs = filter_attribute_rep_pair.into_inner();
                println!("{:?}", inner_pairs);
                let next_peek = inner_pairs.peek().ok_or(anyhow!("{ERROR_MSG}"))?;
                let prefix = if next_peek.as_rule() == Rule::content_with_msa {
                    let rule = 
                        inner_pairs
                            .next()
                            .ok_or(anyhow!("{ERROR_MSG}"))?
                            .into_inner()
                            .next()
                            .ok_or(anyhow!("{ERROR_MSG}"))?;
                    Some(Self::parse_content_with_msa(rule)?)
                } else {
                    None
                };
                let attribute_name = Self::parse_string_with_naming_convention(
                    &inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_name_and_type_separator = Self::parse_content_with_msa(
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
                let attributes_separator = Self::parse_content_with_msa(
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
                let next_peek = inner_pairs.peek();
                let suffix =
                    if let Some(next_peek) = next_peek {
                        if next_peek.as_rule() == Rule::content_with_msa {
                            let rule = 
                                inner_pairs
                                    .next()
                                    .ok_or(anyhow!("{ERROR_MSG}"))?
                                    .into_inner()
                                    .next()
                                    .ok_or(anyhow!("{ERROR_MSG}"))?;
                            Some(Self::parse_content_with_msa(rule)?)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
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
                println!("{:?}", inner_pairs);
                let next_peek = inner_pairs.peek().ok_or(anyhow!("{ERROR_MSG}"))?;
                let prefix = if next_peek.as_rule() == Rule::content_with_msa {
                    Some(Self::parse_content_with_msa(
                        inner_pairs
                            .next()
                            .ok_or(anyhow!("{ERROR_MSG}"))?
                            .into_inner()
                            .next()
                            .ok_or(anyhow!("{ERROR_MSG}"))?
                            .into_inner()
                            .next()
                            .ok_or(anyhow!("{ERROR_MSG}"))?)?)
                } else {
                    None
                };
                let attribute_name = Self::parse_string_with_naming_convention(
                    &inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attribute_name_twice_separator = Self::parse_content_with_msa(
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
                let attributes_separator = Self::parse_content_with_msa(
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
                let next_peek = inner_pairs.peek();
                let suffix = if let Some(next_peek) = next_peek {
                    if next_peek.as_rule() == Rule::content_with_msa {
                        Some(Self::parse_content_with_msa(
                            inner_pairs
                                .next()
                                .ok_or(anyhow!("{ERROR_MSG}"))?
                                .into_inner()
                                .next()
                                .ok_or(anyhow!("{ERROR_MSG}"))?
                                .into_inner()
                                .next()
                                .ok_or(anyhow!("{ERROR_MSG}"))?,
                                
                        )?)
                    } else {
                        None
                    }
                } else {
                    None
                };
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
                println!("{:?}", inner_pairs);
                let next_peek = inner_pairs.peek().ok_or(anyhow!("{ERROR_MSG}"))?;
                let prefix = if next_peek.as_rule() == Rule::content_with_msa {
                    let rule = 
                        inner_pairs
                            .next()
                            .ok_or(anyhow!("{ERROR_MSG}"))?
                            .into_inner()
                            .next()
                            .ok_or(anyhow!("{ERROR_MSG}"))?;

                    Some(Self::parse_content_with_msa(rule)?)
                } else {
                    None
                };
                let attribute_name = Self::parse_string_with_naming_convention(
                    &inner_pairs
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?
                        .into_inner()
                        .next()
                        .ok_or(anyhow!("{ERROR_MSG}"))?,
                )?;
                let attributes_separator = Self::parse_content_with_msa(
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
                let next_peek = inner_pairs.peek();
                let suffix = if let Some(next_peek) = next_peek {
                    if next_peek.as_rule() == Rule::content_with_msa {
                        Some(Self::parse_content_with_msa(
                            inner_pairs
                                .next()
                                .ok_or(anyhow!("{ERROR_MSG}"))?
                                .into_inner()
                                .next()
                                .ok_or(anyhow!("{ERROR_MSG}"))?,
                        )?)
                    } else {
                        None
                    }
                } else {
                    None
                };
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
    
    pub fn parse_filter_rep(filter_rep_pair: Pair<Rule>) -> anyhow::Result<FilterRep> {
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
            Rule::content_with_msa => {
                let content_with_msa_pair = filter_body_pair
                    .into_inner()
                    .next()
                    .ok_or(anyhow!("{ERROR_MSG}"))?;
                let content_with_msa = Self::parse_content_with_msa(content_with_msa_pair)?;
                Ok(FilterBody::ContentWithMSA(content_with_msa))
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
}