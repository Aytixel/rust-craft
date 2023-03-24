mod monster_spawn_light_level;

use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::BufReader;

use super::DeserializeFolder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

impl DeserializeFolder for DimensionType {
    fn deserialize_folder() -> Result<HashMap<String, Self>, String>
    where
        Self: Sized,
    {
        let mut hashmap = HashMap::new();

        for file in read_dir("./data/minecraft/dimension_type/").unwrap() {
            if let Ok(file) = file {
                let file_name = file.file_name();
                let file_name = file_name.to_str().unwrap();

                if file_name.ends_with(".json") {
                    let file = File::open(file.path()).map_err(|e| e.to_string())?;
                    let reader = BufReader::new(file);

                    hashmap.insert(
                        file_name[..file_name.len() - 5].to_string(),
                        serde_json::from_reader(reader).map_err(|e| e.to_string())?,
                    );
                }
            }
        }

        Ok(hashmap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_folder() {
        println!("{:#?}", DimensionType::deserialize_folder().unwrap());
    }
}
