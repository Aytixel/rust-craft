mod pools;

use std::fmt::Debug;

use datapack_macro::DeserializeJsonFolder;
use serde::Deserialize;

#[derive(Deserialize, DeserializeJsonFolder)]
pub struct LootTable {
    r#type: Option<String>,
    pools: Option<Vec<pools::Pool>>,
}

impl Debug for LootTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Structure");

        if let Some(r#type) = &self.r#type {
            s.field("type", &r#type);
        }

        if let Some(pools) = &self.pools {
            s.field("pools", &pools);
        }

        s.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_folder() {
        println!(
            "{:#?}",
            LootTable::deserialize_json_folder("./data/minecraft/loot_tables/").unwrap()
        );
    }
}
