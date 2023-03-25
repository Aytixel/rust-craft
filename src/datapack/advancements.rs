mod criteria;
mod display;

use std::{collections::HashMap, fmt::Debug};

use datapack_macro::DeserializeFolder;
use serde::Deserialize;

#[derive(Deserialize, DeserializeFolder)]
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
pub struct Advancements {
    adventure: HashMap<String, Advancement>,
    end: HashMap<String, Advancement>,
    husbandry: HashMap<String, Advancement>,
    nether: HashMap<String, Advancement>,
    recipes: HashMap<String, Advancement>,
    story: HashMap<String, Advancement>,
}

impl Advancements {
    fn deserialize_folder(path: &str) -> Result<Self, String> {
        Ok(Self {
            adventure: Advancement::deserialize_folder(format!("{path}/adventure/").as_str())?,
            end: Advancement::deserialize_folder(format!("{path}/end/").as_str())?,
            husbandry: Advancement::deserialize_folder(format!("{path}/husbandry/").as_str())?,
            nether: Advancement::deserialize_folder(format!("{path}/nether/").as_str())?,
            recipes: Advancement::deserialize_folder(format!("{path}/recipes/").as_str())?,
            story: Advancement::deserialize_folder(format!("{path}/story/").as_str())?,
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
