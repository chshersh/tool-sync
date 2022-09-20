use std::collections::BTreeMap;

use crate::model::asset_name::AssetName;
use crate::model::tool::{ToolInfo, ToolInfoTag};

/// Get info about known tools from a hardcoded database
pub fn lookup_tool(tool_name: &str) -> Option<ToolInfo> {
    let mut known_db = build_db();
    known_db.remove(tool_name)
}

pub fn build_db() -> BTreeMap<String, ToolInfo> {
    let mut tools: BTreeMap<String, ToolInfo> = BTreeMap::new();

    insert_tool_into!(
        tools,
        "bat",
        {
            owner: "sharkdp",
            repo: "bat",
            exe_name: "bat",
            asset_name: {
                linux: "x86_64-unknown-linux-musl",
                macos: "x86_64-apple-darwin",
                windows: "x86_64-pc-windows-msvc",
            },
            tag: ToolInfoTag::Latest,
        },
    );
    insert_tool_into!(
        tools,
        "difftastic",
        {
            owner: "Wilfred",
            repo: "difftastic",
            exe_name: "difft",
            asset_name: {
                linux: "x86_64-unknown-linux-gnu",
                macos: "x86_64-apple-darwin",
                windows: "x86_64-pc-windows-msvc",
            },
            tag: ToolInfoTag::Latest,
        },
    );
    insert_tool_into!(
        tools,
        "exa",
        {
            owner: "ogham",
            repo: "exa",
            exe_name: "exa",
            asset_name: {
                linux: "linux-x86_64-musl",
                macos: "macos-x86_64",
                windows: "",
            },
            tag: ToolInfoTag::Latest,
        },
    );
    insert_tool_into!(
        tools,
        "fd",
        {
            owner: "sharkdp",
            repo: "fd",
            exe_name: "fd",
            asset_name: {
                linux: "x86_64-unknown-linux-musl",
                macos: "x86_64-apple-darwin",
                windows: "x86_64-pc-windows-msvc",
            },
            tag: ToolInfoTag::Latest,
        },
    );
    insert_tool_into!(
        tools,
        "ripgrep",
        {
            owner: "BurntSushi",
            repo: "ripgrep",
            exe_name: "rg",
            asset_name: {
                linux: "unknown-linux-musl",
                macos: "apple-darwin",
                windows: "x86_64-pc-windows-msvc",
            },
            tag: ToolInfoTag::Latest,
        },
    );
    insert_tool_into!(
        tools,
        "tool-sync",
        {
            owner: "chshersh",
            repo: "tool-sync",
            exe_name: "tool",
            asset_name: {
                linux: "x86_64-unknown-linux-gnu",
                macos: "x86_64-apple-darwin",
                windows: "x86_64-pc-windows-msvc",
            },
            tag: ToolInfoTag::Latest,
        },
    );
    insert_tool_into!(
        tools,
        "github",
        {
            owner: "cli",
            repo: "cli",
            exe_name: "gh",
            asset_name: {
                linux: "linux_amd64.tar.gz",
                macos: "macOS_amd64",
                windows: "windows_amd64.zip",
            },
            tag: ToolInfoTag::Latest,
        },
    );
    insert_tool_into!(
        tools,
        "github",
        {
            owner: "cli",
            repo: "cli",
            exe_name: "gh",
            asset_name: {
                linux: "linux_amd64.tar.gz",
                macos: "macOS_amd64",
                windows: "windows_amd64.zip",
            },
            tag: ToolInfoTag::Latest,
        },
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

macro_rules! insert_tool_into {
    // trailing commas
    (
        $tools:expr,
        $tool_name:expr,
        {
            owner: $owner:expr,
            repo: $repo:expr,
            exe_name: $exe_name:expr,
            asset_name: {
                linux: $linux:expr,
                macos: $macos:expr,
                windows: $windows:expr,
            },
            tag: $tag:expr,
        },
    ) => {
        let mut targets: Vec<Option<String>> = vec![$linux, $macos, $windows]
            .into_iter()
            .map(|item| {
                if item.is_empty() {
                    None
                } else {
                    Some(item.to_string())
                }
            })
            .collect();

        $tools.insert(
            $tool_name.to_string(),
            ToolInfo {
                owner: $owner.to_string(),
                repo: $repo.to_string(),
                exe_name: $exe_name.to_string(),
                asset_name: AssetName {
                    linux: targets.remove(0),
                    macos: targets.remove(0),
                    windows: targets.remove(0),
                },
                tag: $tag,
            },
        )
    };

    // no commas
    (
        $tools:expr,
        $tool_name:expr,
        {
            owner: $owner:expr,
            repo: $repo:expr,
            exe_name: $exe_name:expr,
            asset_name: {
                linux: $linux:expr,
                macos: $macos:expr,
                windows: $windows:expr
            }
            tag: $tag:expr
        }
    ) => {
        let mut targets: Vec<Option<String>> = vec![$linux, $macos, $windows]
            .into_iter()
            .map(|item| {
                if item.is_empty() {
                    None
                } else {
                    Some(item.to_string())
                }
            })
            .collect();

        $tools.insert(
            $tool_name.to_string(),
            ToolInfo {
                owner: $owner.to_string(),
                repo: $repo.to_string(),
                exe_name: $exe_name.to_string(),
                asset_name: AssetName {
                    linux: targets.remove(0),
                    macos: targets.remove(0),
                    windows: targets.remove(0),
                },
                tag: $tag,
            },
        )
    };
}

// Macros normally have to be defined before they get used, this statement makes it usable above
// its definition as well.
use insert_tool_into;
