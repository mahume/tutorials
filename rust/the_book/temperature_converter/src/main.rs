use std::fmt::{self, Display, Formatter};
use std::io;

struct Temperature {
    value: f64,
    unit: TemperatureUnit,
}
impl Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let unit_symbol = match self.unit {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
        };
        write!(f, "{:.2} {}", self.value, unit_symbol)
    }
}

enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl Display for TemperatureUnit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let unit_symbol = match self {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
        };
        write!(f, "{}", unit_symbol)
    }
}

fn main() {
    let mut buffer = String::new();

    let f = Temperature {
        value: 0.0,
        unit: TemperatureUnit::Fahrenheit,
    };
    let c = Temperature {
        value: 0.0,
        unit: TemperatureUnit::Celsius,
    };

    println!("Starting Unit");
    println!("{} or {}", f.unit, c.unit);

    io::stdin().read_line(&mut buffer).expect("Huh?!");

    println!("You input: {}", buffer.trim());
}
