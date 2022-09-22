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
    let mut tools = BTreeMap::<&'static str, StaticToolInfo>::new();

    tools.insert(
        "bat",
        StaticToolInfo {
            owner: "sharkdp",
            repo: "bat",
            exe_name: "bat",
            linux: "x86_64-unknown-linux-musl",
            macos: "x86_64-apple-darwin",
            windows: "x86_64-pc-windows-msvc",
            tag: ToolInfoTag::Latest,
        },
    );
    tools.insert(
        "difftastic",
        StaticToolInfo {
            owner: "Wilfred",
            repo: "difftastic",
            exe_name: "difft",
            linux: "x86_64-unknown-linux-gnu",
            macos: "x86_64-apple-darwin",
            windows: "x86_64-pc-windows-msvc",
            tag: ToolInfoTag::Latest,
        },
    );
    tools.insert(
        "exa",
        StaticToolInfo {
            owner: "ogham",
            repo: "exa",
            exe_name: "exa",
            linux: "linux-x86_64-musl",
            macos: "macos-x86_64",
            windows: NOT_SUPPORTED,
            tag: ToolInfoTag::Latest,
        },
    );
    tools.insert(
        "fd",
        StaticToolInfo {
            owner: "sharkdp",
            repo: "fd",
            exe_name: "fd",
            linux: "x86_64-unknown-linux-musl",
            macos: "x86_64-apple-darwin",
            windows: "x86_64-pc-windows-msvc",
            tag: ToolInfoTag::Latest,
        },
    );
    tools.insert(
        "ripgrep",
        StaticToolInfo {
            owner: "BurntSushi",
            repo: "ripgrep",
            exe_name: "rg",
            linux: "unknown-linux-musl",
            macos: "apple-darwin",
            windows: "x86_64-pc-windows-msvc",
            tag: ToolInfoTag::Latest,
        },
    );
    tools.insert(
        "tool-sync",
        StaticToolInfo {
            owner: "chshersh",
            repo: "tool-sync",
            exe_name: "tool",
            linux: "x86_64-unknown-linux-gnu.tar.gz",
            macos: "x86_64-apple-darwin.tar.gz",
            windows: "x86_64-pc-windows-msvc.zip",
            tag: ToolInfoTag::Latest,
        },
    );
    tools.insert(
        "github",
        StaticToolInfo {
            owner: "cli",
            repo: "cli",
            exe_name: "gh",
            linux: "linux_amd64.tar.gz",
            macos: "macOS_amd64",
            windows: "windows_amd64.zip",
            tag: ToolInfoTag::Latest,
        },
    );
    //tools.insert(
    //    "tokei",
    //    StaticToolInfo {
    //        owner: "XAMPPRocky",
    //        repo: "tokei",
    //        exe_name: "tokei",
    //        linux: "x86_64-unknown-linux-musl",
    //        macos: "apple-darwin",
    //        windows: "x86_64-pc-windows-msvc",
    //        tag: ToolInfoTag::Latest,
    //    }
    //);

    BTreeMap::from_iter(
        tools
            .into_iter()
            .map(|(name, tool_info)| (name.to_owned(), tool_info.into())),
    )
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

impl From<StaticToolInfo> for ToolInfo {
    fn from(static_tool_info: StaticToolInfo) -> Self {
        ToolInfo {
            owner: static_tool_info.owner.to_string(),
            repo: static_tool_info.repo.to_string(),
            exe_name: static_tool_info.exe_name.to_string(),
            asset_name: AssetName {
                linux: from_supported_asset(static_tool_info.linux),
                macos: from_supported_asset(static_tool_info.macos),
                windows: from_supported_asset(static_tool_info.windows),
            },
            tag: static_tool_info.tag,
        }
    }
}

#[inline]
fn from_supported_asset(asset_name: &str) -> Option<String> {
    if asset_name == NOT_SUPPORTED {
        None
    } else {
        Some(asset_name.to_string())
    }
}
