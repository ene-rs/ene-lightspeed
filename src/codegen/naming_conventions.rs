use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToTrainCase, ToUpperCamelCase,
};

#[derive(Debug)]
pub enum NamingConvention {
    KebabCase,
    LowerCamelCase,
    UpperCamelCase,
    ShoutyKebabCase,
    ShoutySnakeCase,
    SnakeCase,
    TitleCase,
    TrainCase,
}

impl NamingConvention {
    pub fn transform_string_to_naming_convention(&self, s: &str) -> String {
        match self {
            NamingConvention::KebabCase => s.to_kebab_case(),
            NamingConvention::LowerCamelCase => s.to_lower_camel_case(),
            NamingConvention::UpperCamelCase => s.to_upper_camel_case(),
            NamingConvention::ShoutyKebabCase => s.to_shouty_kebab_case(),
            NamingConvention::ShoutySnakeCase => s.to_shouty_snake_case(),
            NamingConvention::SnakeCase => s.to_snake_case(),
            NamingConvention::TitleCase => s.to_title_case(),
            NamingConvention::TrainCase => s.to_train_case(),
        }
    }
}

pub fn detect_naming_convention(s: &str) -> NamingConvention {
    if s == &s.to_kebab_case() {
        NamingConvention::KebabCase
    } else if s == &s.to_lower_camel_case() {
        NamingConvention::LowerCamelCase
    } else if s == &s.to_upper_camel_case() {
        NamingConvention::UpperCamelCase
    } else if s == &s.to_snake_case() {
        NamingConvention::SnakeCase
    } else if s == &s.to_shouty_kebab_case() {
        NamingConvention::ShoutyKebabCase
    } else if s == &s.to_shouty_snake_case() {
        NamingConvention::ShoutySnakeCase
    } else if s == &s.to_title_case() {
        NamingConvention::TitleCase
    } else if s == &s.to_train_case() {
        NamingConvention::TrainCase
    } else {
        panic!("Could not detect the naming convention used")
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn experiment() {
        let entity = "entity";
        println!("naming_convention: {:?}", super::detect_naming_convention(entity));
    }
}