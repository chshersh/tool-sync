use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use toml::{Value, map::Map};

use crate::model::asset_name::AssetName;
use crate::config::schema::{Config, ConfigAsset};

pub fn parse_config(config_path: &PathBuf) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(config_path)?;
    let value = contents.parse::<Value>()?;

    match decode_config(value) {
        None => Err(format!("Error decoding TOML file: {}", config_path.display()).into()),
        Some(tool) => Ok(tool),
    }
}

fn decode_config(toml: Value) -> Option<Config> {
    let t_store_directory = toml["store_directory"].as_str()?;
    let store_directory = PathBuf::from(t_store_directory);

    let mut tools = HashMap::new();

    for (key, val) in toml.as_table()?.iter() {
        if let Value::Table(table) = val {
            tools.insert(key.clone(), decode_config_asset(table));
        }
    }

    Some(Config {
        store_directory,
        tools: tools,
    })
}

fn decode_config_asset(table: &Map<String, Value>) -> ConfigAsset {
    let owner = str_by_key(table, "owner");
    let repo = str_by_key(table, "repo");
    let exe_name = str_by_key(table, "exe_name");
    let asset_name = decode_asset_name(table);

    ConfigAsset {
        owner,
        repo,
        exe_name,
        asset_name
    }
}

fn decode_asset_name(table: &Map<String, Value>) -> AssetName {
    match table.get("asset_name").and_then(|t| t.as_table()) {
        None => AssetName {
            linux: None,
            macos: None,
            windows: None,
        },

        Some(table) => {
           let linux = str_by_key(table, "linux");
           let macos = str_by_key(table, "macos");
           let windows = str_by_key(table, "windows");

           AssetName {
              linux,
              macos,
              windows
           }
        }
    }
}

fn str_by_key(table: &Map<String, Value>, key: &str) -> Option<String> {
    table
    .get(key)
    .and_then(|v| v.as_str())
    .map(String::from)
}