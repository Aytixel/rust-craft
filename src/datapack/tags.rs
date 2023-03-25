use std::collections::HashMap;

use datapack_macro::DeserializeFolder;
use serde::Deserialize;

#[derive(Debug, Deserialize, DeserializeFolder)]
pub struct Tag {
    values: Vec<String>,
}

#[derive(Debug)]
pub struct BannerPatternTags {
    root: HashMap<String, Tag>,
    pattern_item: HashMap<String, Tag>,
}

impl BannerPatternTags {
    fn deserialize_folder(path: &str) -> Result<Self, String> {
        Ok(Self {
            root: Tag::deserialize_folder(path)?,
            pattern_item: Tag::deserialize_folder(format!("{path}/pattern_item/").as_str())?,
        })
    }
}

#[derive(Debug)]
pub struct BlocksTags {
    root: HashMap<String, Tag>,
    mineable: HashMap<String, Tag>,
}

impl BlocksTags {
    fn deserialize_folder(path: &str) -> Result<Self, String> {
        Ok(Self {
            root: Tag::deserialize_folder(path)?,
            mineable: Tag::deserialize_folder(format!("{path}/mineable/").as_str())?,
        })
    }
}

#[derive(Debug)]
pub struct BiomeTags {
    root: HashMap<String, Tag>,
    has_structure: HashMap<String, Tag>,
}

impl BiomeTags {
    fn deserialize_folder(path: &str) -> Result<Self, String> {
        Ok(Self {
            root: Tag::deserialize_folder(path)?,
            has_structure: Tag::deserialize_folder(format!("{path}/has_structure/").as_str())?,
        })
    }
}

#[derive(Debug)]
pub struct WorldgenTags {
    biome: BiomeTags,
    flat_level_generator_preset: HashMap<String, Tag>,
    structure: HashMap<String, Tag>,
    world_preset: HashMap<String, Tag>,
}

impl WorldgenTags {
    fn deserialize_folder(path: &str) -> Result<Self, String> {
        Ok(Self {
            biome: BiomeTags::deserialize_folder(format!("{path}/biome/").as_str())?,
            flat_level_generator_preset: Tag::deserialize_folder(
                format!("{path}/flat_level_generator_preset/").as_str(),
            )?,
            structure: Tag::deserialize_folder(format!("{path}/structure/").as_str())?,
            world_preset: Tag::deserialize_folder(format!("{path}/world_preset/").as_str())?,
        })
    }
}

#[derive(Debug)]
pub struct Tags {
    banner_pattern: BannerPatternTags,
    blocks: BlocksTags,
    cat_variant: HashMap<String, Tag>,
    damage_type: HashMap<String, Tag>,
    entity_types: HashMap<String, Tag>,
    fluids: HashMap<String, Tag>,
    game_events: HashMap<String, Tag>,
    instrument: HashMap<String, Tag>,
    items: HashMap<String, Tag>,
    painting_variant: HashMap<String, Tag>,
    point_of_interest_type: HashMap<String, Tag>,
    worldgen: WorldgenTags,
}

impl Tags {
    pub fn deserialize_folder(path: &str) -> Result<Self, String> {
        Ok(Self {
            banner_pattern: BannerPatternTags::deserialize_folder(
                format!("{path}/banner_pattern/").as_str(),
            )?,
            blocks: BlocksTags::deserialize_folder(format!("{path}/blocks/").as_str())?,
            cat_variant: Tag::deserialize_folder(format!("{path}/cat_variant/").as_str())?,
            damage_type: Tag::deserialize_folder(format!("{path}/damage_type/").as_str())?,
            entity_types: Tag::deserialize_folder(format!("{path}/entity_types/").as_str())?,
            fluids: Tag::deserialize_folder(format!("{path}/fluids/").as_str())?,
            game_events: Tag::deserialize_folder(format!("{path}/game_events/").as_str())?,
            instrument: Tag::deserialize_folder(format!("{path}/instrument/").as_str())?,
            items: Tag::deserialize_folder(format!("{path}/items/").as_str())?,
            painting_variant: Tag::deserialize_folder(
                format!("{path}/painting_variant/").as_str(),
            )?,
            point_of_interest_type: Tag::deserialize_folder(
                format!("{path}/point_of_interest_type/").as_str(),
            )?,
            worldgen: WorldgenTags::deserialize_folder(format!("{path}/worldgen/").as_str())?,
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
            Tags::deserialize_folder("./data/minecraft/tags/").unwrap()
        );
    }
}
