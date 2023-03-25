mod criteria;
mod display;

use std::{collections::HashMap, fmt::Debug};

use datapack_macro::DeserializeJsonFolder;
use serde::Deserialize;

#[derive(Deserialize, DeserializeJsonFolder)]
pub struct Advancement {
    parent: Option<String>,
    criteria: HashMap<String, criteria::Criteria>,
    display: Option<display::Display>,
    requirements: Vec<Vec<String>>,
}

impl Debug for Advancement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Advancement");

        if let Some(parent) = &self.parent {
            s.field("parent", &parent);
        }

        s.field("criteria", &self.criteria);

        if let Some(display) = &self.display {
            s.field("display", &display);
        }

        s.field("requirements", &self.requirements).finish()
    }
}

#[derive(Debug)]
pub struct RecipesAdvancements {
    root: HashMap<String, Advancement>,
    brewing: HashMap<String, Advancement>,
    building_blocks: HashMap<String, Advancement>,
    combat: HashMap<String, Advancement>,
    decorations: HashMap<String, Advancement>,
    food: HashMap<String, Advancement>,
    misc: HashMap<String, Advancement>,
    redstone: HashMap<String, Advancement>,
    tools: HashMap<String, Advancement>,
    transportation: HashMap<String, Advancement>,
}

impl RecipesAdvancements {
    fn deserialize_folder(path: &str) -> Result<Self, String> {
        Ok(Self {
            root: Advancement::deserialize_json_folder(path)?,
            brewing: Advancement::deserialize_json_folder(format!("{path}/brewing/").as_str())?,
            building_blocks: Advancement::deserialize_json_folder(
                format!("{path}/building_blocks/").as_str(),
            )?,
            combat: Advancement::deserialize_json_folder(format!("{path}/combat/").as_str())?,
            decorations: Advancement::deserialize_json_folder(
                format!("{path}/decorations/").as_str(),
            )?,
            food: Advancement::deserialize_json_folder(format!("{path}/food/").as_str())?,
            misc: Advancement::deserialize_json_folder(format!("{path}/misc/").as_str())?,
            redstone: Advancement::deserialize_json_folder(format!("{path}/redstone/").as_str())?,
            tools: Advancement::deserialize_json_folder(format!("{path}/tools/").as_str())?,
            transportation: Advancement::deserialize_json_folder(
                format!("{path}/transportation/").as_str(),
            )?,
        })
    }
}

#[derive(Debug)]
pub struct Advancements {
    adventure: HashMap<String, Advancement>,
    end: HashMap<String, Advancement>,
    husbandry: HashMap<String, Advancement>,
    nether: HashMap<String, Advancement>,
    recipes: RecipesAdvancements,
    story: HashMap<String, Advancement>,
}

impl Advancements {
    pub fn deserialize_folder(path: &str) -> Result<Self, String> {
        Ok(Self {
            adventure: Advancement::deserialize_json_folder(format!("{path}/adventure/").as_str())?,
            end: Advancement::deserialize_json_folder(format!("{path}/end/").as_str())?,
            husbandry: Advancement::deserialize_json_folder(format!("{path}/husbandry/").as_str())?,
            nether: Advancement::deserialize_json_folder(format!("{path}/nether/").as_str())?,
            recipes: RecipesAdvancements::deserialize_folder(format!("{path}/recipes/").as_str())?,
            story: Advancement::deserialize_json_folder(format!("{path}/story/").as_str())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_folder() {
        println!(
            "{:#?}",
            Advancements::deserialize_folder("./data/minecraft/advancements/").unwrap()
        );
    }
}
