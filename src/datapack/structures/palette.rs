use std::fmt::Debug;

use quartz_nbt::NbtCompound;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Block {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Properties")]
    properties: Option<NbtCompound>,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Block");

        s.field("Name", &self.name);

        if let Some(properties) = &self.properties {
            s.field("Properties", &properties);
        }

        s.finish()
    }
}
