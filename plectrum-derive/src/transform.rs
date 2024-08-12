use convert_case::{Case, Casing};

/// Abstraction for transforming the tokens representing the variants
/// of enum that implement the `plectrum::Enum` trait.
///
/// It's simply a wrapper on top of `convert_case::Case`
#[derive(Debug)]
pub(crate) struct CaseTransform {
    case: Case,
}

impl From<String> for CaseTransform {
    fn from(source: String) -> Self {
        let s = source.as_str();
        let case = match s {
            "UPPER CASE" => Case::Upper,
            "lower case" => Case::Lower,
            "Title Case" => Case::Title,
            "camelCase" => Case::Camel,
            "UpperCamelCase" => Case::UpperCamel,
            "snake_case" => Case::Snake,
            "UPPER_SNAKE_CASE" => Case::UpperSnake,
            "kebab-case" => Case::Kebab,
            "UPPER-KEBAB-CASE" => Case::UpperKebab,
            "Train-Case" => Case::Train,
            "flatcase" => Case::Flat,
            "UPPERFLATCASE" => Case::UpperFlat,
            _ => panic!("Invalid value for 'rename_all' transform: {}", s),
        };
        Self { case }
    }
}

impl CaseTransform {
    pub(crate) fn convert(&self, s: &str) -> String {
        s.to_case(self.case)
    }
}
