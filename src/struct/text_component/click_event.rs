use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "action", content = "value")]
pub enum ClickEvent {
    #[serde(rename = "open_url")]
    OpenUrl(Url),
    #[serde(rename = "run_command")]
    RunCommand(String),
    #[serde(rename = "suggest_command")]
    SuggestCommand(String),
    #[serde(rename = "change_page")]
    ChangePage(u32),
    #[serde(rename = "copy_to_clipboard")]
    CopyToClipboard(String),
}
