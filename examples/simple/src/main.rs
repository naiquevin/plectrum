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

