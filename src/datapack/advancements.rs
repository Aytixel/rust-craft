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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_folder() {
        println!(
            "{:#?}",
            Advancement::deserialize_json_folder("./data/minecraft/advancements/").unwrap()
        );
    }
}
