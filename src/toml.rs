use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use toml::{Value, map::Map};

use crate::tool::{Tool, ToolDetails};

pub fn parse_config(config_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config_path)?;
    let value = contents.parse::<Value>()?;

    println!("{:?}", value);

    println!("{:?}", decode_tool(value));
 
    Ok(())
}

fn decode_tool(toml: Value) -> Option<Tool> {
    let t_store_directory = toml["store_directory"].as_str()?;
    let store_directory = PathBuf::from(t_store_directory);

    let mut tools = HashMap::new();

    for (key, val) in toml.as_table()?.iter() {
        if let Value::Table(table) = val {
            tools.insert(key.clone(), decode_tool_details(table));
        }
    }

    Some(Tool {
        store_directory,
        tools: tools,
    })
}

fn decode_tool_details(table: &Map<String, Value>) -> ToolDetails {
    let url = table
            .get("url")
            .and_then(|v| v.as_str())
            .map(String::from);

    ToolDetails {
        url
    }
}