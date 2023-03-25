mod advancements;
mod chat_type;
mod damage_type;
mod dimension_type;
mod items;
mod recipes;
mod structures;
mod tags;
mod translate;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Datapack {
    advancements: HashMap<String, advancements::Advancement>,
    chat_type: HashMap<String, chat_type::ChatType>,
    damage_type: HashMap<String, damage_type::DamageType>,
    dimension_type: HashMap<String, dimension_type::DimensionType>,
    recipes: HashMap<String, recipes::Recipes>,
    structures: HashMap<String, structures::Structure>,
    tags: HashMap<String, tags::Tag>,
}

impl Datapack {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            advancements: advancements::Advancement::deserialize_json_folder(
                "./data/minecraft/advancements/",
            )?,
            chat_type: chat_type::ChatType::deserialize_json_folder("./data/minecraft/chat_type/")?,
            damage_type: damage_type::DamageType::deserialize_json_folder(
                "./data/minecraft/damage_type/",
            )?,
            dimension_type: dimension_type::DimensionType::deserialize_json_folder(
                "./data/minecraft/dimension_type/",
            )?,
            recipes: recipes::Recipes::deserialize_json_folder("./data/minecraft/recipes/")?,
            structures: structures::Structure::deserialize_nbt_folder(
                "./data/minecraft/structures/",
            )?,
            tags: tags::Tag::deserialize_json_folder("./data/minecraft/tags/")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        println!("{:#?}", Datapack::new().unwrap());
    }
}
