use std::fmt::Debug;

use quartz_nbt::NbtCompound;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Entitiy {
    pos: [f64; 3],
    #[serde(rename = "blockPos")]
    block_pos: [i32; 3],
    nbt: Option<NbtCompound>,
}

impl Debug for Entitiy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Entitiy");

        s.field("pos", &self.pos).field("blockPos", &self.block_pos);

        if let Some(nbt) = &self.nbt {
            s.field("nbt", &nbt);
        }

        s.finish()
    }
}
