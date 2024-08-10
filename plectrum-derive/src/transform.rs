use convert_case::{Case, Casing};

/// Abstraction for transforming the tokens representing the variants
/// of enum that implement the `plectrum::Enum` trait.
///
/// It's simply a wrapper on top of `convert_case::Case`
#[derive(Debug)]
pub(crate) enum Transform {
    UpperCase,
    LowerCase,
    TitleCase,
    CamelCase,
    UpperCamelCase,
    SnakeCase,
    UpperSnakeCase,
    KebabCase,
    UpperKebabCase,
    TrainCase,
    FlatCase,
    UpperFlatCase,
}

impl From<String> for Transform {
    fn from(source: String) -> Self {
        let s = source.as_str();
        match s {
            "UPPER CASE" => Self::UpperCase,
            "lower case" => Self::LowerCase,
            "Title Case" => Self::TitleCase,
            "camelCase" => Self::CamelCase,
            "UpperCamelCase" => Self::UpperCamelCase,
            "snake_case" => Self::SnakeCase,
            "UPPER_SNAKE_CASE" => Self::UpperSnakeCase,
            "kebab-case" => Self::KebabCase,
            "UPPER-KEBAB-CASE" => Self::UpperKebabCase,
            "Train-Case" => Self::TrainCase,
            "flatcase" => Self::FlatCase,
            "UPPERFLATCASE" => Self::UpperFlatCase,
            _ => panic!("Invalid value for 'rename_all' transform: {}", s),
        }
    }
}

impl Transform {
    pub(crate) fn convert(&self, s: &str) -> String {
        let case = match self {
            Transform::UpperCase => Case::Upper,
            Transform::LowerCase => Case::Lower,
            Transform::TitleCase => Case::Title,
            Transform::CamelCase => Case::Camel,
            Transform::UpperCamelCase => Case::UpperCamel,
            Transform::SnakeCase => Case::Snake,
            Transform::UpperSnakeCase => Case::UpperSnake,
            Transform::KebabCase => Case::Kebab,
            Transform::UpperKebabCase => Case::UpperKebab,
            Transform::TrainCase => Case::Train,
            Transform::FlatCase => Case::Flat,
            Transform::UpperFlatCase => Case::UpperFlat,
        };
        s.to_case(case)
    }
}
