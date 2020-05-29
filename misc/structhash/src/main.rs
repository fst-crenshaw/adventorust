use clap::{App, Arg};
use std;

#[derive(Debug, Default)]
struct Sailor {
    foo: String,
    fox: u32,
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
            Arg::with_name("sailor")
                .short("s")
                .long("sailor")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    let stuff: Vec<KvPair> = matches
        .values_of("sailor")
        .unwrap()
        .map(|s| s.parse().unwrap())
        .collect();

    let sailor = {
        let mut sailor = Sailor::default();
        for pair in stuff {
            let KvPair { key, value } = pair;
            match key.as_str() {
                "foo" => sailor.foo = value,
                "fox" => sailor.fox = value.parse().unwrap(),
                _ => todo!(),
            }
        }
        sailor
    };

    dbg!(sailor);
}
