use anyhow::Error;
use toml;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
struct Flavors {
    kind: Kind,
    filling: Filling,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Kind {
    Vanilla,
    Chocolate,	
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Filling {
    Nuts,
    Fruit,
}

#[derive(Debug, Deserialize)]
struct Globals {
    number: u32,
    name: String,
    flavors: Flavors,
}

#[derive(Debug, Deserialize)]
struct Config {
    globals: Globals,
}
    
fn main() -> Result<(), Error> {

    let config_toml = std::fs::read_to_string("config.toml")?;

    println!("Contents of Config File");
    //dbg!(config_toml);

    
    let config: Config = toml::from_str(&config_toml)?;

    dbg!(config);
    println!("Hello, world!");

   
    Ok(())
}
