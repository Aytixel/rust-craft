use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum TextColor {
    #[serde(rename = "black")]
    #[serde(alias = "#000000")]
    Black,
    #[serde(rename = "dark_blue")]
    #[serde(alias = "#0000aa")]
    DarkBlue,
    #[serde(rename = "dark_green")]
    #[serde(alias = "#00aa00")]
    DarkGreen,
    #[serde(rename = "dark_aqua")]
    #[serde(alias = "#00aaaa")]
    DarkCyan,
    #[serde(rename = "dark_red")]
    #[serde(alias = "#aa0000")]
    DarkRed,
    #[serde(rename = "dark_purple")]
    #[serde(alias = "#aa00aa")]
    Purple,
    #[serde(rename = "gold")]
    #[serde(alias = "#ffaa00")]
    Gold,
    #[serde(rename = "gray")]
    #[serde(alias = "#aaaaaa")]
    Gray,
    #[serde(rename = "dark_gray")]
    #[serde(alias = "#555555")]
    DarkGray,
    #[serde(rename = "blue")]
    #[serde(alias = "#5555ff")]
    Blue,
    #[serde(rename = "green")]
    #[serde(alias = "#55ff55")]
    Green,
    #[serde(rename = "aqua")]
    #[serde(alias = "#55ffff")]
    Cyan,
    #[serde(rename = "red")]
    #[serde(alias = "#ff5555")]
    Red,
    #[serde(rename = "light_purple")]
    #[serde(alias = "#ff55ff")]
    Pink,
    #[serde(rename = "yellow")]
    #[serde(alias = "#ffff55")]
    Yellow,
    #[serde(rename = "white")]
    #[serde(alias = "#ffffff")]
    White,
    #[serde(rename = "")]
    Reset,
    #[serde(untagged)]
    Custom(String),
}
