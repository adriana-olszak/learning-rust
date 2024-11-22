use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}


#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}


impl Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}", self.real, self.imag)
    }
}

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}


impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,   "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue,
            self.red, self.green, self.blue)
    }
}

/// Prints hello world message!
fn main() {
  println!("Rust adventure begins!");

  println!("{} days", 132);
  println!("{} days", "Doom");

  println!("{animal} {excercise} {sport}",
      animal="dog",
          excercise="squat",
          sport="pole dance"
  );
println!("My name is {0}, {1} {0}", "Bond", "James");

let pi = 3.141592;
println!("Pi is roughly {:.3}", pi);
println!("Hello structure prints debug {:#?}", Structure(123));
println!("Hello structure implements display {}", Structure(123));

let point = Point2D { x: 3.3, y: 7.2 };
println!("Compare points:");
println!("Display: {}", point);
println!("Debug: {:?}", point);

let complex = Complex {real: 3.3, imag: 7.2 };
println!("Display: {}", complex);
println!("Debug: {:?}", complex);


let v = List(vec![1, 2, 3]);
println!("{}", v);


for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }

for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }
}
