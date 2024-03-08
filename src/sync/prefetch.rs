use console::{style, Emoji};
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};

use std::collections::BTreeMap;
use std::fmt::Display;

use super::configure::configure_tool;
use crate::config::schema::ConfigAsset;
use crate::infra::client::Client;
use crate::model::release::AssetError;
use crate::model::repo::RepoError;
use crate::model::tool::{Tool, ToolAsset};

const PREFETCH: Emoji<'_, '_> = Emoji("🔄 ", "-> ");
const ERROR: Emoji<'_, '_> = Emoji("❌ ", "x ");
const PACKAGE: Emoji<'_, '_> = Emoji("📦 ", "# ");

struct PrefetchProgress {
    pb: ProgressBar,
    total_count: usize,
}

impl PrefetchProgress {
    fn new(total_count: usize) -> PrefetchProgress {
        let pb = create_prefetch_progress_bar();
        PrefetchProgress { pb, total_count }
    }

    fn update_message(&self, already_completed: usize) {
        let remaining_count = self.total_count - already_completed;

        if remaining_count == 0 {
            self.pb.set_message("All done!");
            self.pb.finish()
        } else {
            self.pb.set_message(format!(
                "Fetching info about {} tools (this may take a few seconds)...",
                remaining_count
            ))
        }
    }

    /// This method can take in any type that implements the [`Display`] trait
    fn expected_err_msg<Message: Display>(&self, tool_name: &str, msg: Message) {
        let tool = format!("{}", style(tool_name).cyan().bold());
        self.pb.println(format!("{} {} {}", ERROR, tool, msg))
    }

    /// This method can take in any type that implements the [`Display`] trait
    fn unexpected_err_msg<Message: Display>(&self, tool_name: &str, msg: Message) {
        let tool = format!("{}", style(tool_name).cyan().bold());
        let err_msg = format!(
            r#"{emoji} {tool} {msg}

If you think you see this error by a 'tool-sync' mistake,
don't hesitate to open an issue:

    * https://github.com/chshersh/tool-sync/issues/new"#,
            emoji = ERROR,
            tool = tool,
            msg = msg,
        );

        self.pb.println(err_msg);
    }

    fn finish(&self) {
        self.pb.finish()
    }
}

/// Fetch information about all the tool from the configuration. This function
/// combines two steps:
///
///   1. Resolving all the required fields from `ConfigAsset`.
///   2. Fetching release and asset info from GitHub.
pub fn prefetch(tools: BTreeMap<String, ConfigAsset>) -> Vec<ToolAsset> {
    let total_count = tools.len();

    let prefetch_progress = PrefetchProgress::new(total_count);
    prefetch_progress.update_message(0);

    let tool_assets: Vec<ToolAsset> = tools
        .iter()
        .enumerate()
        .filter_map(|(index, (tool_name, config_asset))| {
            //println!("{:?}", config_asset.proxy.clone());
            prefetch_tool(
                tool_name,
                config_asset,
                &prefetch_progress,
                index,
                config_asset.proxy.clone(),
            )
        })
        .collect();

    prefetch_progress.finish();

    let estimated_download_size: u64 = tool_assets.iter().map(|ta| ta.asset.size).sum();
    let size = HumanBytes(estimated_download_size);
    eprintln!(
        "{emoji} Estimated total download size: {size}",
        emoji = PACKAGE,
        size = size
    );

    tool_assets
}

fn prefetch_tool(
    tool_name: &str,
    config_asset: &ConfigAsset,
    prefetch_progress: &PrefetchProgress,
    current_index: usize,
    proxy: Option<ureq::Proxy>,
) -> Option<ToolAsset> {
    // indexes start with 0 so we add 1 to calculate already fetched tools
    let already_completed = current_index + 1;

    match configure_tool(tool_name, config_asset) {
        Tool::Error(e) => {
            prefetch_progress.expected_err_msg(tool_name, e);
            prefetch_progress.update_message(already_completed);
            None
        }
        Tool::Known(tool_info) => {
            let client = Client {
                owner: tool_info.owner.clone(),
                repo: tool_info.repo.clone(),
                version: tool_info.tag.to_str_version(),
                proxy,
            };

            match client.fetch_release_info() {
                Err(e) => {
                    if let Some(ureq::Error::Status(404, _)) = e.downcast_ref::<ureq::Error>() {
                        prefetch_progress.unexpected_err_msg(
                            tool_name,
                            RepoError::NotFound {
                                owner: tool_info.owner,
                                repo: tool_info.repo,
                                tag: tool_info.tag,
                            },
                        );
                    } else {
                        prefetch_progress.unexpected_err_msg(tool_name, e);
                    }
                    // do some other processing
                    prefetch_progress.update_message(already_completed);
                    None
                }
                Ok(release) => match tool_info.select_asset(&release.assets) {
                    Err(err) => match err {
                        AssetError::MultipleFound(_) => {
                            prefetch_progress.expected_err_msg(tool_name, err);
                            prefetch_progress.update_message(already_completed);
                            None
                        }
                        _ => {
                            prefetch_progress.unexpected_err_msg(tool_name, err);
                            prefetch_progress.update_message(already_completed);
                            None
                        }
                    },
                    Ok(asset) => {
                        let tool_asset = ToolAsset {
                            tool_name: String::from(tool_name),
                            tag: release.tag_name,
                            exe_name: tool_info.exe_name,
                            asset,
                            client,
                        };

                        prefetch_progress.update_message(already_completed);

                        Some(tool_asset)
                    }
                },
            }
        }
    }
}

fn create_prefetch_progress_bar() -> ProgressBar {
    let message_style = ProgressStyle::with_template("{prefix} {msg}").unwrap();

    ProgressBar::new(100)
        .with_style(message_style)
        .with_prefix(format!("{}", PREFETCH))
}
