mod monster_spawn_light_level;

use datapack_macro::DeserializeFolder;
use serde::Deserialize;

#[derive(Debug, Deserialize, DeserializeFolder)]
pub struct DimensionType {
    ambient_light: f64,
    bed_works: bool,
    coordinate_scale: f64,
    effects: String,
    fixed_time: Option<u32>,
    has_ceiling: bool,
    has_raids: bool,
    has_skylight: bool,
    height: u32,
    logical_height: u32,
    min_y: i32,
    infiniburn: String,
    monster_spawn_block_light_limit: i32,
    monster_spawn_light_level: monster_spawn_light_level::MonsterSpawnLightLevel,
    natural: bool,
    piglin_safe: bool,
    respawn_anchor_works: bool,
    ultrawarm: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_folder() {
        println!(
            "{:#?}",
            DimensionType::deserialize_folder("./data/minecraft/dimension_type/").unwrap()
        );
    }
}
