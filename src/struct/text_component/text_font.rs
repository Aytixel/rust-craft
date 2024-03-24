use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum TextFont {
    #[serde(rename = "minecraft:default")]
    Default,
    #[serde(rename = "minecraft:uniform")]
    GNUUnifont,
    #[serde(rename = "minecraft:alt")]
    StandardGalacticAlphabet,
    #[serde(rename = "minecraft:illageralt")]
    Illageralt,
    #[serde(untagged)]
    Custom(String),
}
