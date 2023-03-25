mod blocks;
mod entities;
mod palette;

use std::fmt::Debug;

use datapack_macro::DeserializeNbtFolder;
use serde::Deserialize;

#[derive(Debug, Deserialize, DeserializeNbtFolder)]
pub struct Structure {
    #[serde(rename = "DataVersion")]
    data_version: i32,
    size: [i32; 3],
    blocks: Vec<blocks::Block>,
    palette: Option<Vec<palette::Block>>,
    palettes: Option<Vec<Vec<palette::Block>>>,
    entities: Vec<entities::Entitiy>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_folder() {
        println!(
            "{:#?}",
            Structure::deserialize_nbt_folder("./data/minecraft/structures/").unwrap()
        );
    }
}
