use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use toml::{map::Map, Value};

use crate::config::schema::{Config, ConfigAsset};
use crate::infra::err;
use crate::model::asset_name::AssetName;
use crate::model::os::OS;

#[derive(Debug, PartialEq, Eq)]
pub enum TomlError {
    IO(String),
    Parse(toml::de::Error),
    Decode,
}

impl Display for TomlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlError::IO(e) => write!(f, "[IO Error] {}", e),
            TomlError::Parse(e) => write!(f, "[Parsing Error] {}", e),
            TomlError::Decode => write!(f, "[Decode Error]"),
        }
    }
}

pub fn with_parsed_file<F: FnOnce(Config)>(config_path: PathBuf, on_success: F) {
    match parse_file(&config_path) {
        Ok(config) => {
            on_success(config);
        }
        Err(e) => {
            err::abort_with(format!(
                "Error parsing configuration at path {}: {}",
                config_path.display(),
                e
            ));
        }
    }
}

fn parse_file(config_path: &PathBuf) -> Result<Config, TomlError> {
    let contents = fs::read_to_string(config_path).map_err(|e| TomlError::IO(format!("{}", e)))?;

    parse_string(&contents)
}

fn parse_string(contents: &str) -> Result<Config, TomlError> {
    contents
        .parse::<Value>()
        .map_err(TomlError::Parse)
        .and_then(|toml| match decode_config(toml) {
            None => Err(TomlError::Decode),
            Some(config) => Ok(config),
        })
}

fn decode_config(toml: Value) -> Option<Config> {
    let str_store_directory = toml.get("store_directory")?.as_str()?;
    let store_directory = String::from(str_store_directory);

    let mut tools = BTreeMap::new();

    for (key, val) in toml.as_table()?.iter() {
        if let Value::Table(table) = val {
            tools.insert(key.clone(), decode_config_asset(table));
        }
    }

    Some(Config {
        store_directory,
        tools,
    })
}

fn decode_config_asset(table: &Map<String, Value>) -> ConfigAsset {
    let owner = str_by_key(table, "owner");
    let repo = str_by_key(table, "repo");
    let exe_name = str_by_key(table, "exe_name");
    let asset_name = decode_asset_name(table);
    let tag = str_by_key(table, "tag");

    ConfigAsset {
        owner,
        repo,
        exe_name,
        asset_name,
        tag,
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
            let linux = str_by_key(table, OS::Linux.to_string().as_str());
            let macos = str_by_key(table, OS::MacOS.to_string().as_str());
            let windows = str_by_key(table, OS::Windows.to_string().as_str());

            AssetName {
                linux,
                macos,
                windows,
            }
        }
    }
}

fn str_by_key(table: &Map<String, Value>, key: &str) -> Option<String> {
    table.get(key).and_then(|v| v.as_str()).map(String::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_error_display_io() {
        let toml_error = TomlError::IO(String::from("some file error!"));

        assert_eq!(
            String::from("[IO Error] some file error!"),
            toml_error.to_string()
        );
    }

    #[test]
    fn test_toml_error_display_parse() {
        let broken_toml_str: String = "broken toml".into();
        match parse_string(&broken_toml_str) {
            Err(error) => {
                assert_eq!(
                    String::from(
                        "[Parsing Error] expected an equals, found an identifier at line 1 column 8"
                    ),
                    error.to_string()
                );
            }
            Ok(_) => unreachable!(),
        };
    }

    #[test]
    fn test_toml_error_display_decode() {
        let toml_error = TomlError::Decode;
        assert_eq!(String::from("[Decode Error]"), toml_error.to_string());
    }

    #[test]
    fn test_parse_file_correct_output() {
        let result = std::panic::catch_unwind(|| {
            let test_config_path = PathBuf::from("tests/sync-full.toml");
            parse_file(&test_config_path).expect("This should not fail")
        });

        if let Ok(config) = result {
            assert_eq!(String::from("sync-full"), config.store_directory);
        };
    }

    #[test]
    fn test_parse_file_error() {
        let test_config_path = PathBuf::from("src/main.rs");
        match parse_file(&test_config_path) {
            Ok(_) => {
                assert!(false, "Unexpected succces")
            }
            Err(_) => {
                assert!(true, "Exepected a parsing error")
            }
        };
    }

    #[test]
    fn empty_file() {
        let toml = "";
        let res = parse_string(toml);

        assert_eq!(res, Err(TomlError::Decode));
    }

    #[test]
    fn store_directory_is_dotted() {
        let toml = "store.directory = \"pancake\"";
        let res = parse_string(toml);

        assert_eq!(res, Err(TomlError::Decode));
    }

    #[test]
    fn store_directory_is_a_number() {
        let toml = "store_directory = 42";
        let res = parse_string(toml);

        assert_eq!(res, Err(TomlError::Decode));
    }

    #[test]
    fn only_store_directory() {
        let toml = "store_directory = \"pancake\"";
        let res = parse_string(toml);

        let cfg = Config {
            store_directory: String::from("pancake"),
            tools: BTreeMap::new(),
        };

        assert_eq!(res, Ok(cfg));
    }

    #[test]
    fn single_empty_tool() {
        let toml = r#"
            store_directory = "pancake"

            [ripgrep]
        "#;

        let res = parse_string(toml);

        let cfg = Config {
            store_directory: String::from("pancake"),
            tools: BTreeMap::from([(
                "ripgrep".to_owned(),
                ConfigAsset {
                    owner: None,
                    repo: None,
                    exe_name: None,
                    asset_name: AssetName {
                        linux: None,
                        macos: None,
                        windows: None,
                    },
                    tag: None,
                },
            )]),
        };

        assert_eq!(res, Ok(cfg));
    }

    #[test]
    fn two_empty_tools() {
        let toml = r#"
            store_directory = "pancake"

            [ripgrep]
            [bat]
        "#;

        let res = parse_string(toml);

        let cfg = Config {
            store_directory: String::from("pancake"),
            tools: BTreeMap::from([
                (
                    "ripgrep".to_owned(),
                    ConfigAsset {
                        owner: None,
                        repo: None,
                        exe_name: None,
                        asset_name: AssetName {
                            linux: None,
                            macos: None,
                            windows: None,
                        },
                        tag: None,
                    },
                ),
                (
                    "bat".to_owned(),
                    ConfigAsset {
                        owner: None,
                        repo: None,
                        exe_name: None,
                        asset_name: AssetName {
                            linux: None,
                            macos: None,
                            windows: None,
                        },
                        tag: None,
                    },
                ),
            ]),
        };

        assert_eq!(res, Ok(cfg));
    }

    #[test]
    fn single_partial_tool() {
        let toml = r#"
            store_directory = "pancake"

            [ripgrep]
            owner = "me"
            asset_name.linux = "R2D2"
        "#;

        let res = parse_string(toml);

        let cfg = Config {
            store_directory: String::from("pancake"),
            tools: BTreeMap::from([(
                "ripgrep".to_owned(),
                ConfigAsset {
                    owner: Some("me".to_owned()),
                    repo: None,
                    exe_name: None,
                    asset_name: AssetName {
                        linux: Some("R2D2".to_owned()),
                        macos: None,
                        windows: None,
                    },
                    tag: None,
                },
            )]),
        };

        assert_eq!(res, Ok(cfg));
    }

    #[test]
    fn single_full_tool() {
        let toml = r#"
            store_directory = "pancake"

            [ripgrep]
            owner = "me"
            repo = "some_repo"
            exe_name = "rg"
            asset_name.linux = "R2D2"
            asset_name.macos = "C3-PO"
            asset_name.windows = "IG-88"
            tag = "4.2.0"
        "#;

        let res = parse_string(toml);

        let cfg = Config {
            store_directory: String::from("pancake"),
            tools: BTreeMap::from([(
                "ripgrep".to_owned(),
                ConfigAsset {
                    owner: Some("me".to_owned()),
                    repo: Some("some_repo".to_owned()),
                    exe_name: Some("rg".to_owned()),
                    asset_name: AssetName {
                        linux: Some("R2D2".to_owned()),
                        macos: Some("C3-PO".to_owned()),
                        windows: Some("IG-88".to_owned()),
                    },
                    tag: Some("4.2.0".to_owned()),
                },
            )]),
        };

        assert_eq!(res, Ok(cfg));
    }
}
