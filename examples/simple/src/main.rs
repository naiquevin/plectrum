use plectrum::Enum;
use plectrum_derive::Plectrum;

#[derive(Debug, Plectrum)]
enum Color {
    Red,
    Green,
    Yellow,
}

fn main() {
    println!("Playground for testing the Plectrum macro");
    println!("{:?}", Color::values());
    println!("{}", Color::Red.value());
    println!("{:?}", Color::from_value("Green"));
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use plectrum::Enum;

    use super::Color;

    #[test]
    fn test_plectrum_macro() {
        let red = "Red";
        let green = "Green";
        let yellow = "Yellow";

        // Test for the `values` fn
        assert_eq!(
            HashSet::from_iter(vec![red, green, yellow]),
            Color::values(),
        );

        // Tests for the `value` method
        assert_eq!(red, Color::Red.value());
        assert_eq!(green, Color::Green.value());
        assert_eq!(yellow, Color::Yellow.value());

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
    }
}
