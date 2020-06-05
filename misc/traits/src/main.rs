use clap::{App, Arg};
use std;

trait Shape {
    fn new(diameter: u32, height: u32) -> Self;
    fn perimeter(&self) -> u32;
    fn area(&self) -> u32;
}

#[derive(Debug, Default)]
struct Params {
    name: String,
    diameter: u32,
    height: u32,
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

    let stuff: Vec<KvPair> = matches
        .values_of("params")
        .unwrap()
	.iter()
	.map(|s| s.parse().unwrap())
	.collect();

    let params = {
        let mut params = Params::default();
        for pair in stuff {
            let KvPair { key, value } = pair;
            match key.as_str() {
		"name" => params.name = value,
                "diameter" => params.diameter = value.parse().unwrap(),
                "height" => params.height = value.parse().unwrap(),
                _ => todo!(),
            }
        }
	params
    };

    dbg!(params);
}
