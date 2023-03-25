mod monster_spawn_light_level;

use std::fmt::Debug;

use datapack_macro::DeserializeJsonFolder;
use serde::Deserialize;

#[derive(Deserialize, DeserializeJsonFolder)]
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
    monster_spawn_light_level: monster_spawn_light_level::MonsterSpawnLightLevelVariant,
    natural: bool,
    piglin_safe: bool,
    respawn_anchor_works: bool,
    ultrawarm: bool,
}

impl Debug for DimensionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("DimensionType");

        s.field("ambient_light", &self.ambient_light)
            .field("bed_works", &self.bed_works)
            .field("coordinate_scale", &self.coordinate_scale)
            .field("effects", &self.effects);

        if let Some(fixed_time) = self.fixed_time {
            s.field("fixed_time", &fixed_time);
        }

        s.field("has_ceiling", &self.has_ceiling)
            .field("has_raids", &self.has_raids)
            .field("has_skylight", &self.has_skylight)
            .field("height", &self.height)
            .field("logical_height", &self.logical_height)
            .field("min_y", &self.min_y)
            .field("infiniburn", &self.infiniburn)
            .field(
                "monster_spawn_block_light_limit",
                &self.monster_spawn_block_light_limit,
            )
            .field("monster_spawn_light_level", &self.monster_spawn_light_level)
            .field("natural", &self.natural)
            .field("piglin_safe", &self.piglin_safe)
            .field("respawn_anchor_works", &self.respawn_anchor_works)
            .field("ultrawarm", &self.ultrawarm)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_folder() {
        println!(
            "{:#?}",
            DimensionType::deserialize_json_folder("./data/minecraft/dimension_type/").unwrap()
        );
    }
}
