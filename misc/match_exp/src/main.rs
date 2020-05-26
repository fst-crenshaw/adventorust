use anyhow::{anyhow, Error};

fn main() -> Result<(), Error> {

    let val = "Hello, world!";
    
    println!("{}", val);

    match val.len() {

	5 => return Ok(()),
	_ => return Err(anyhow!("Uhoh")),
    }
    
}
