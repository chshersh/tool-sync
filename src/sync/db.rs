use std::collections::BTreeMap;

use crate::model::asset_name::AssetName;
use crate::model::tool::{ToolInfo, ToolInfoTag};

const NOT_SUPPORTED: &str = "NOT_SUPPORTED";

/// Get info about known tools from a hardcoded database
pub fn lookup_tool(tool_name: &str) -> Option<ToolInfo> {
    let mut known_db = build_db();
    known_db.remove(tool_name)
}

pub fn build_db() -> BTreeMap<String, ToolInfo> {
    let mut tools: BTreeMap<String, ToolInfo> = BTreeMap::new();

    tools.insert(
        "bat".into(),
        StaticToolInfo {
            owner: "sharkdp",
            repo: "bat",
            exe_name: "bat",
            linux: "x86_64-unknown-linux-musl",
            macos: "x86_64-apple-darwin",
            windows: "x86_64-pc-windows-msvc",
            tag: ToolInfoTag::Latest,
        }
        .into(),
    );
    tools.insert(
        "difftastic".into(),
        StaticToolInfo {
            owner: "Wilfred",
            repo: "difftastic",
            exe_name: "difft",
            linux: "x86_64-unknown-linux-gnu",
            macos: "x86_64-apple-darwin",
            windows: "x86_64-pc-windows-msvc",
            tag: ToolInfoTag::Latest,
        }
        .into(),
    );
    tools.insert(
        "exa".into(),
        StaticToolInfo {
            owner: "ogham",
            repo: "exa",
            exe_name: "exa",
            linux: "linux-x86_64-musl",
            macos: "macos-x86_64",
            windows: NOT_SUPPORTED,
            tag: ToolInfoTag::Latest,
        }
        .into(),
    );
    tools.insert(
        "fd".into(),
        StaticToolInfo {
            owner: "sharkdp",
            repo: "fd",
            exe_name: "fd",
            linux: "x86_64-unknown-linux-musl",
            macos: "x86_64-apple-darwin",
            windows: "x86_64-pc-windows-msvc",
            tag: ToolInfoTag::Latest,
        }
        .into(),
    );
    tools.insert(
        "ripgrep".into(),
        StaticToolInfo {
            owner: "BurntSushi",
            repo: "ripgrep",
            exe_name: "rg",
            linux: "unknown-linux-musl",
            macos: "apple-darwin",
            windows: "x86_64-pc-windows-msvc",
            tag: ToolInfoTag::Latest,
        }
        .into(),
    );
    tools.insert(
        "tool-sync".into(),
        StaticToolInfo {
            owner: "chshersh",
            repo: "tool-sync",
            exe_name: "tool",
            linux: "x86_64-unknown-linux-gnu.tar.gz",
            macos: "x86_64-apple-darwin.tar.gz",
            windows: "x86_64-pc-windows-msvc.zip",
            tag: ToolInfoTag::Latest,
        }
        .into(),
    );
    tools.insert(
        "github".into(),
        StaticToolInfo {
            owner: "cli",
            repo: "cli",
            exe_name: "gh",
            linux: "linux_amd64.tar.gz",
            macos: "macOS_amd64",
            windows: "windows_amd64.zip",
            tag: ToolInfoTag::Latest,
        }
        .into(),
    );
    // tools.insert("tokei", ToolInfo {
    //     owner: "XAMPPRocky".to_string(),
    //     repo: "tokei".to_string(),
    //     exe_name: "tokei".to_string(),
    //     asset_name: AssetName {
    //         linux: Some("x86_64-unknown-linux-musl".to_string()),
    //         macos: Some("apple-darwin".to_string()),
    //         windows: Some("x86_64-pc-windows-msvc".to_string()),
    //       }
    //     tag: ToolInfoTag::Latest,
    // }));
    //
    tools
}

/// Format tool names of the database using a mutating formatting function
/// The result is something like this (depending on a function)
///
/// ```toml
/// # [bat]
/// # [exa]
/// ```
pub fn fmt_tool_names<F: FnMut(&String) -> String>(fmt_tool: F) -> String {
    build_db()
        .keys()
        .map(fmt_tool)
        .collect::<Vec<String>>()
        .join("\n")
}

struct StaticToolInfo {
    /// Tool name
    //pub name: &'static str,

    /// GitHub repository author
    pub owner: &'static str,

    /// GitHub repository name
    pub repo: &'static str,

    /// Executable name inside the .tar.gz or .zip archive
    pub exe_name: &'static str,

    /// Version tag
    pub tag: ToolInfoTag,

    pub linux: &'static str,
    pub macos: &'static str,
    pub windows: &'static str,
}

//impl Into<ToolInfo> for StaticToolInfo {
//    fn into(self) -> ToolInfo {
//        ToolInfo {
//            owner: self.owner.to_string(),
//            repo: self.repo.to_string(),
//            exe_name: self.exe_name.to_string(),
//            tag: self.tag,
//            asset_name: AssetName {
//                linux: to_supported_asset(self.linux.to_string()),
//                macos: to_supported_asset(self.macos.to_string()),
//                windows: to_supported_asset(self.windows.to_string()),
//            },
//        }
//    }
//}

//#[inline]
//fn to_supported_asset(asset_name: String) -> Option<String> {
//    if asset_name == NOT_SUPPORTED { None } else { Some(asset_name) }
//}

impl From<StaticToolInfo> for ToolInfo {
    fn from(static_tool_info: StaticToolInfo) -> Self {
        Self {
            owner: static_tool_info.owner.to_string(),
            repo: static_tool_info.repo.to_string(),
            exe_name: static_tool_info.exe_name.to_string(),
            asset_name: AssetName {
                linux: from_supported_asset(static_tool_info.linux.to_string()),
                macos: from_supported_asset(static_tool_info.macos.to_string()),
                windows: from_supported_asset(static_tool_info.windows.to_string()),
            },
            tag: static_tool_info.tag,
        }
    }
}

#[inline]
fn from_supported_asset(asset_name: String) -> Option<String> {
    if asset_name == NOT_SUPPORTED {
        None
    } else {
        Some(asset_name)
    }
}
