#![feature(try_trait)] // This only works with rust nightly, required for using ? on NoneError

use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("The input value was too small")]
    ValueTooSmall(String, std::option::NoneError),
}

impl From<std::option::NoneError> for Error {
    fn from(error: std::option::NoneError) -> Self {
        Error::ValueTooSmall(error)
    }
}

fn main() -> Result<(), Error> {
    let mut s = fs::read_to_string("input.txt").unwrap();
    s = s.trim().to_string();

    let mut total = 0;

    for module_mass in s.split('\n') {
        let module_mass = module_mass.parse::<u32>().unwrap() / 3;
        let module_mass = module_mass.checked_sub(2)?;
        total = total + module_mass;
    }

    println!("Total Mass! : {}", total);
    Ok(())
}
