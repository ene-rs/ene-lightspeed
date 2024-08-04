#[derive(Debug, Clone)]
pub struct EntityTemplate {
    pub entity_content: Vec<EntityTemplateContent>,
}
#[derive(Debug, Clone)]
pub enum Content {
    Static(String),
    EntityName(StringWithNamingConvention),
    EntityPlural(StringWithNamingConvention),
    PrimaryKey(StringWithNamingConvention),
}

#[derive(Debug, Clone)]
pub enum EntityTemplateContent {
    Content(Content),
    AttributeRep(AttributeRep),
    FilterRep(FilterRep),
}
#[derive(Debug, Clone)]
pub enum AttributeRep {
    NamesAndTypesPresent {
        prefix: Content,
        attribute_name: StringWithNamingConvention,
        attribute_name_and_type_separator: Content,
        attribute_type: StringWithNamingConvention,
        attributes_separator: Content,
        suffix: Content,
    },
    NamesPresent {
        prefix: Content,
        attribute_name: StringWithNamingConvention,
        attributes_separator: Content,
        suffix: Content,
    },
    NamesTwicePresent {
        prefix: Content,
        attribute_name: StringWithNamingConvention,
        attribute_name_twice_separator: Content,
        attributes_separator: Content,
        suffix: Content,
    },

}

#[derive(Debug, Clone)]
pub enum ContentWithMSFA {
    Content(Content),
    MostSpecificFilterAttributeName(StringWithNamingConvention),
    MostSpecificFilterAttributeType(StringWithNamingConvention),
}

#[derive(Debug, Clone)]
pub enum FilterRep {
    ConsecutiveFilters {
        filter: Filter
    },
    SeparatedFilters {
        filter: Filter,
        separator: Content,
    },
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub content: Vec<FilterBody>
}
#[derive(Debug, Clone)]
pub enum FilterBody {
    ContentWithMSFA(ContentWithMSFA),
    FilterAttributeRep(FilterAttributeRep),
}

#[derive(Debug, Clone)]
pub enum FilterAttributeRep {
    NamesAndTypesPresent {
        prefix: Content,
        attribute_name: StringWithNamingConvention,
        attribute_name_and_type_separator: Content,
        attribute_type: StringWithNamingConvention,
        attributes_separator: Content,
        suffix: Content,
    },
    NamesPresent {
        prefix: Content,
        attribute_name: StringWithNamingConvention,
        attributes_separator: Content,
        suffix: Content,
    },
    NamesTwicePresent {
        prefix: Content,
        attribute_name: StringWithNamingConvention,
        attribute_name_twice_separator: Content,
        attributes_separator: Content,
        suffix: Content,
    },

}

#[derive(Debug, Clone)]
pub enum StringWithNamingConvention {
    KebabCase(String),
    LowerCamelCase(String),
    UpperCamelCase(String),
    ShoutyKebabCase(String),
    ShoutySnakeCase(String),
    SnakeCase(String),
    TitleCase(String),
    TrainCase(String),
}
