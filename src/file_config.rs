use std::error::Error;
use std::fs;
use std::path::PathBuf;
use toml::Value;

pub fn parse_config(config_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config_path)?;
    let value = contents.parse::<Value>().unwrap();

    println!("{:?}", value);
 
    Ok(())
}