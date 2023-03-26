mod blocks;
mod entities;
mod palette;

use std::fmt::Debug;

use datapack_macro::DeserializeNbtFolder;
use serde::Deserialize;

#[derive(Deserialize, DeserializeNbtFolder)]
pub struct Structure {
    #[serde(rename = "DataVersion")]
    data_version: i32,
    size: [i32; 3],
    blocks: Vec<blocks::Block>,
    palette: Option<Vec<palette::Block>>,
    palettes: Option<Vec<Vec<palette::Block>>>,
    entities: Vec<entities::Entitiy>,
}

impl Debug for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Structure");

        s.field("DataVersion", &self.data_version)
            .field("size", &self.size)
            .field("blocks", &self.blocks);

        if let Some(palette) = &self.palette {
            s.field("palette", &palette);
        }

        if let Some(palettes) = &self.palettes {
            s.field("palettes", &palettes);
        }

        s.field("entities", &self.entities).finish()
    }
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
