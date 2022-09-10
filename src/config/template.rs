/// This file only holds the template that is used to generate a default .tools.toml.
// This file is generated by the build.rs script. If you want to edit this file change the template
// there.

pub const CONFIG_TEMPLATE: &str = r##"# This config file was generated for version 0.1.0
#
# # tool-sync default configuration file
# https://github.com/chshersh/tool-sync
# This file was automatically generated by tool-sync
#####################################################
#
#
# store_directory = "$HOME/.local/bin"
#
# tool-sync provides native support for some of the tools without the need to configure them
# Uncomment the tools you want to have them
#
# [bat]
# [difftastic]
# [fd]
# [ripgrep]
#
# To add configuration for other tools these are the config options:
# [ripgrep]
#        owner     = "BurntSushi"
#        repo      = "ripgrep"
#        exe_name  = "rg"
#
#        # Uncomment to download a specific version or tag.
#        # Without this tag latest will be used
#        # tag       = "13.0.0"
#
#
# Asset name to download on linux OSes
# asset_name.linux = "x86_64-unknown-linux-musl"
#
# uncomment if you want to install on macOS as well
# asset_name.macos = "apple-darwin"
#
# uncomment if you want to install on Windows as well
# asset_name.windows = "x86_64-pc-windows-msvc"
"##;
