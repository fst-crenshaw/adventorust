use clap::{App, Arg};
use num_traits::pow;
use std;

/// This command line application accepts the name of a shape --
/// either circle or square -- and then calculates the area of
/// that shape based on the supplied n.

/// Example:
///   $ cargo run -- --params diameter=12 name=square
///       144
///
///   $ cargo run -- --params diameter=12 name=circle
///       113.097

/// Shape: this trait defines the generic method to calculate the area
/// of a shape.
trait Shape {
    fn area(&self) -> f32;
}

/// ShapeCreate: To create a shape polymorphically, use a separate
/// creation trait.
trait ShapeCreate {
    fn new(param: u32) -> Self;
}

/// create: A dispatch method, parameterized by T, to create a
/// new kind of shape, T.
fn create<T: ShapeCreate>(param: u32) -> T {
    T::new(param)
}

/// Trait implementation for Circle
struct Circle {
    diameter: u32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
	3.141592 * pow(self.diameter / 2,2) as f32
    }
}

impl ShapeCreate for Circle {
    fn new(diameter: u32) -> Self {
	Circle {
	    diameter,
	}
    }
}

/// Trait implementation for Square
struct Square {
    side: u32,
}

impl Shape for Square {
    fn area(&self) -> f32 {
	(self.side * self.side) as f32
    }    
}

impl ShapeCreate for Square {
    fn new(side: u32) -> Self {
	Square {
	    side, /*: diameter, */
	}
    }
}

/// Params
#[derive(Debug, Default)]
struct Params {
    name: String,
    n: u32,
}

#[derive(Debug, Default)]
struct KvPair {
    key: String,
    value: String,
}

impl std::str::FromStr for KvPair {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // _ --> "elision"
        match s.split('=').collect::<Vec<_>>().as_slice() {
            // "punning"
            [key, value] => Ok(Self {
                key: key.to_string(),
                value: value.to_string(),
            }),
            _ => Err(()),
        }
    }
}

fn main() {

    // Collect the command line parameters using clap.
    let matches = App::new("shapes")
        .arg(
            Arg::with_name("params")
                .short("s")
                .long("params")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    // Create a vector of KvPairs
    let stuff: Vec<KvPair> = matches
        .values_of("params")
        .unwrap()
	.iter()
	.map(|s| s.parse().unwrap())
	.collect();

    // Convert the KvPairs to a Params struct. 
    let params = {
        let mut params = Params::default();
        for KvPair { key, value } in stuff {
            match key.as_str() {
		"name" => params.name = value,
                "n" => params.n = value.parse().unwrap(),
                _ => todo!(),
            }
        }
	params
    };

    // Examine the "name" that was passed in and create the
    // shape equivalent to that name.
    let my_shape: Box<dyn Shape>;
    match params.name.as_str() {
	"circle" =>  {
	    my_shape = Box::new(create::<Circle>(params.n));
	    println!("\nCreating a circle of diameter {} and area {}.\n", params.n, my_shape.area());
	}
	"square" => {
	    my_shape = Box::new(create::<Square>(params.n));
	    println!("\nCreating a square of height {} and area {}.\n", params.n, my_shape.area());
	}
	_ => println!("\nThat shape is not implemented.\n"),
    }
}
