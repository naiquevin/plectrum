use std::collections::HashMap;

use plectrum::{self, Enum, Plectrum};

#[derive(Debug, Plectrum)]
#[plectrum(rename_all = "snake_case")]
enum Color {
    Red,
    Green,
    Yellow,
    DarkBlue,
}

pub struct ColorModel;

impl plectrum::DataSource for ColorModel {
    type Id = u8;

    async fn load(&self) -> Result<HashMap<u8, String>, plectrum::Error> {
        let mut m = HashMap::new();
        m.insert(1, "red".to_owned());
        m.insert(2, "green".to_owned());
        m.insert(3, "yellow".to_owned());
        m.insert(4, "dark_blue".to_owned());
        Ok(m)
    }
}

#[tokio::main]
async fn main() {
    println!("Playground for testing the Plectrum macro");
    dbg!(Color::values());
    dbg!(Color::Red.value());
    dbg!(Color::from_value("green"));

    let model = ColorModel;
    let mapping = plectrum::Mapping::<u8, Color>::load(&model).await.unwrap();
    dbg!(mapping.by_id(1));
    dbg!(mapping.by_value("yellow"));
    dbg!(mapping.get_id(&Color::DarkBlue));
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use plectrum::Enum;

    use super::Color;

    #[test]
    fn test_plectrum_macro() {
        let red = "red";
        let green = "green";
        let yellow = "yellow";
        let darkblue = "dark_blue";

        // Test for the `values` fn
        assert_eq!(
            HashSet::from_iter(vec![red, green, yellow, darkblue]),
            Color::values(),
        );

        // Tests for the `value` method
        assert_eq!(red, Color::Red.value());
        assert_eq!(green, Color::Green.value());
        assert_eq!(yellow, Color::Yellow.value());
        assert_eq!(darkblue, Color::DarkBlue.value());

        // Tests for the `from_value` method
        match Color::from_value(red) {
            Color::Red => assert!(true),
            _ => assert!(false),
        }

        match Color::from_value(green) {
            Color::Green => assert!(true),
            _ => assert!(false),
        }

        match Color::from_value(yellow) {
            Color::Yellow => assert!(true),
            _ => assert!(false),
        }

        match Color::from_value(darkblue) {
            Color::DarkBlue => assert!(true),
            _ => assert!(false),
        }
    }
}
