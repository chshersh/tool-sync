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

    tools.insert(
        "bat".into(),
        ToolInfo {
            owner: "sharkdp".to_string(),
            repo: "bat".to_string(),
            exe_name: "bat".to_string(),
            asset_name: AssetName {
                linux: Some("x86_64-unknown-linux-musl".to_string()),
                macos: Some("x86_64-apple-darwin".to_string()),
                windows: Some("x86_64-pc-windows-msvc".to_string()),
            },
            tag: ToolInfoTag::Latest,
        },
    );
    tools.insert(
        "difftastic".into(),
        ToolInfo {
            owner: "Wilfred".to_string(),
            repo: "difftastic".to_string(),
            exe_name: "difft".to_string(),
            asset_name: AssetName {
                linux: Some("x86_64-unknown-linux-gnu".to_string()),
                macos: Some("x86_64-apple-darwin".to_string()),
                windows: Some("x86_64-pc-windows-msvc".to_string()),
            },
            tag: ToolInfoTag::Latest,
        },
    );
    tools.insert(
        "exa".into(),
        ToolInfo {
            owner: "ogham".to_string(),
            repo: "exa".to_string(),
            exe_name: "exa".to_string(),
            asset_name: AssetName {
                linux: Some("linux-x86_64-musl".to_string()),
                macos: Some("macos-x86_64".to_string()),
                windows: None,
            },
            tag: ToolInfoTag::Latest,
        },
    );
    tools.insert(
        "fd".into(),
        ToolInfo {
            owner: "sharkdp".to_string(),
            repo: "fd".to_string(),
            exe_name: "fd".to_string(),
            asset_name: AssetName {
                linux: Some("x86_64-unknown-linux-musl".to_string()),
                macos: Some("x86_64-apple-darwin".to_string()),
                windows: Some("x86_64-pc-windows-msvc".to_string()),
            },
            tag: ToolInfoTag::Latest,
        },
    );
    tools.insert(
        "ripgrep".into(),
        ToolInfo {
            owner: "BurntSushi".to_string(),
            repo: "ripgrep".to_string(),
            exe_name: "rg".to_string(),
            asset_name: AssetName {
                linux: Some("unknown-linux-musl".to_string()),
                macos: Some("apple-darwin".to_string()),
                windows: Some("x86_64-pc-windows-msvc".to_string()),
            },
            tag: ToolInfoTag::Latest,
        },
    );
    tools.insert(
        "tool-sync".into(),
        ToolInfo {
            owner: "chshersh".to_string(),
            repo: "tool-sync".to_string(),
            exe_name: "tool".to_string(),
            asset_name: AssetName {
                linux: Some("x86_64-unknown-linux-gnu".to_string()),
                macos: Some("x86_64-apple-darwin".to_string()),
                windows: Some("x86_64-pc-windows-msvc".to_string()),
            },
            tag: ToolInfoTag::Latest,
        },
    );
    tools.insert(
        "github".into(),
        ToolInfo {
            owner: "cli".to_string(),
            repo: "cli".to_string(),
            exe_name: "gh".to_string(),
            asset_name: AssetName {
                linux: Some("linux_amd64.tar.gz".to_string()),
                macos: Some("macOS_amd64".to_string()),
                windows: Some("windows_amd64.zip".to_string()),
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
            tag: ToolInfoTag::Latest
        }
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
    //trailing commas
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
        }
    ) => {
        $tools.insert(
            $tool_name.to_string(),
            ToolInfo {
                owner: $owner.to_string(),
                repo: $repo.to_string(),
                exe_name: $exe_name.to_string(),
                asset_name: AssetName {
                    linux: Some($linux.to_string()),
                    macos: Some($macos.to_string()),
                    windows: Some($windows.to_string()),
                },
                tag: $tag,
            },
        )
    };

    // first trailing comma
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
            tag: $tag:expr
        }
    ) => {
        $tools.insert(
            $tool_name.to_string(),
            ToolInfo {
                owner: $owner.to_string(),
                repo: $repo.to_string(),
                exe_name: $exe_name.to_string(),
                asset_name: AssetName {
                    linux: Some($linux.to_string()),
                    macos: Some($macos.to_string()),
                    windows: Some($windows.to_string()),
                },
                tag: $tag,
            },
        )
    };

    // last trailing comma
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
            }
            tag: $tag:expr,
        }
    ) => {
        $tools.insert(
            $tool_name.to_string(),
            ToolInfo {
                owner: $owner.to_string(),
                repo: $repo.to_string(),
                exe_name: $exe_name.to_string(),
                asset_name: AssetName {
                    linux: Some($linux.to_string()),
                    macos: Some($macos.to_string()),
                    windows: Some($windows.to_string()),
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
        $tools.insert(
            $tool_name.to_string(),
            ToolInfo {
                owner: $owner.to_string(),
                repo: $repo.to_string(),
                exe_name: $exe_name.to_string(),
                asset_name: AssetName {
                    linux: Some($linux.to_string()),
                    macos: Some($macos.to_string()),
                    windows: Some($windows.to_string()),
                },
                tag: $tag,
            },
        )
    };
}

// Macros normally have to be defined before they get used, this statement makes it usable above
// its definition as well.
use insert_tool_into;
