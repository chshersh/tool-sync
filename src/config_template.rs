/// This file only holds the template that is used to generate a default .tools.toml.

pub const CONFIG_TEMPLATE: &str = r##"# This file was automatically generated by tool-sync
#
#store_directory = "$HOME/.local/bin"
#
#    [bat]
#        owner =     "sharkdp"
#        repo =      "bat"
#        exe_name =  "bat"
#        tag =       "latest"
#    [exa]
#        owner =     "ogham"
#        repo =      "exa"
#        exe_name =  "exa"
#        tag =       "latest"
#    [fd]
#        owner =     "sharkdp"
#        repo =      "fd"
#        exe_name =  "fd"
#        tag =       "latest"
#    [ripgrep]
#        owner =     "BurntSushi"
#        repo =      "ripgrep"
#        exe_name =  "rg"
#        tag =       "latest"
#"##;
