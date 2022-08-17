use std::collections::HashMap;
use std::path::PathBuf;

/// Stores information about all the configured tools
#[derive(Debug)]
pub struct Tool {
  /// Directory to store all locally downloaded tools
  pub store_directory: PathBuf,

  /// Info about each individual tool
  pub tools: HashMap<String, ToolDetails>,
}

/// Additional details, telling how to download a tool
#[derive(Debug)]
pub struct ToolDetails {
    pub url: Option<String>,
}