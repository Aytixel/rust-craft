mod advancements;
mod chat_type;
mod damage_type;
mod dimension_type;
mod items;
mod loot_tables;
mod recipes;
mod structures;
mod tags;
mod translate;

use hashbrown::hash_map::HashMap;

#[derive(Debug)]
pub struct Datapack {
    advancements: HashMap<String, advancements::Advancement>,
    chat_type: HashMap<String, chat_type::ChatType>,
    damage_type: HashMap<String, damage_type::DamageType>,
    dimension_type: HashMap<String, dimension_type::DimensionType>,
    loot_tables: HashMap<String, loot_tables::LootTable>,
    recipes: HashMap<String, recipes::Recipes>,
    structures: HashMap<String, structures::Structure>,
    tags: HashMap<String, tags::Tag>,
}

impl Datapack {
    pub fn new(path: &str) -> Result<Self, String> {
        Ok(Self {
            advancements: advancements::Advancement::deserialize_json_folder(
                (path.to_string() + "/advancements/").as_str(),
            )?,
            chat_type: chat_type::ChatType::deserialize_json_folder(
                (path.to_string() + "/chat_type/").as_str(),
            )?,
            damage_type: damage_type::DamageType::deserialize_json_folder(
                (path.to_string() + "/damage_type/").as_str(),
            )?,
            dimension_type: dimension_type::DimensionType::deserialize_json_folder(
                (path.to_string() + "/dimension_type/").as_str(),
            )?,
            loot_tables: loot_tables::LootTable::deserialize_json_folder(
                (path.to_string() + "/loot_tables/").as_str(),
            )?,
            recipes: recipes::Recipes::deserialize_json_folder(
                (path.to_string() + "/recipes/").as_str(),
            )?,
            structures: structures::Structure::deserialize_nbt_folder(
                (path.to_string() + "/structures/").as_str(),
            )?,
            tags: tags::Tag::deserialize_json_folder((path.to_string() + "/tags/").as_str())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        println!("{:#?}", Datapack::new("./data/minecraft/").unwrap());
    }
}
