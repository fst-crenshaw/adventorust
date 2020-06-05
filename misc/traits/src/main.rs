use clap::{App, Arg};
use num_traits::pow;
use std;

// Generic way to describe the necessary behavior of a shape.
trait Shape {
    fn new(n: u32) -> Self;
    fn area(&self) -> f32;
}

struct Circle {
    diameter: u32,
}

impl Shape for Circle {
    fn new(diameter: u32) -> Self {
	Circle {
	    diameter,
	}
    }
    fn area(&self) -> f32 {
	3.141592 * pow(self.diameter / 2,2) as f32
    }
}

struct Square {
    side: u32,
}

impl Shape for Square {
    fn new(side: u32) -> Self {
	Square {
	    side, /*: diameter, */
	}
    }
    fn area(&self) -> f32 {
	(self.side * self.side) as f32
    }
    
}


#[derive(Debug, Default)]
struct Params {
    name: String,
    diameter: u32,
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
    let matches = App::new("structhash!")
        .arg(
            Arg::with_name("params")
                .short("s")
                .long("params")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    // dbg!(matches);

    
    let stuff: Vec<KvPair> = matches
        .values_of("params")
        .unwrap()
	.iter()
	.map(|s| s.parse().unwrap())
	.collect();

    // dbg!(stuff);
    
    let params = {
        let mut params = Params::default();
        for KvPair { key, value } in stuff {
            match key.as_str() {
		"name" => params.name = value,
                "diameter" => params.diameter = value.parse().unwrap(),
                _ => todo!(),
            }
        }
	params
    };

    // Examine the "name" that was passed in and create the
    // shape equivalent to that name.
    match params.name {
	"circle" =  println!("Work?"),
	_ = println!("Nope"),
    }
    
    let my_circle = Circle::new(params.diameter);
    let my_square = Square::new(params.diameter);

    dbg!(params);
    dbg!(my_circle.area());
    dbg!(my_square.area());
}
