use std::fmt::Debug;

use quartz_nbt::NbtCompound;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Block {
    pos: [i32; 3],
    state: i32,
    nbt: Option<NbtCompound>,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Block");

        s.field("pos", &self.pos).field("state", &self.state);

        if let Some(nbt) = &self.nbt {
            s.field("nbt", &nbt);
        }

        s.finish()
    }
}
