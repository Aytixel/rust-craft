mod decoration;
mod effects;
mod int_provider;

pub use decoration::*;
pub use effects::*;
pub use int_provider::*;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

use super::TextComponent;

#[derive(Debug, Deserialize, Serialize)]
pub struct RegistryCodec {
    #[serde(flatten)]
    pub registries: HashMap<String, Registery>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Registery {
    pub r#type: String,
    pub value: Vec<RegistryEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegistryEntry {
    pub name: String,
    pub id: i32,
    pub element: RegistryElement,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RegistryElement {
    ArmorTrimMaterial {
        asset_name: String,
        ingredient: String,
        item_model_index: f32,
        override_armor_materials: Option<HashMap<String, String>>,
        description: TextComponent,
    },
    ArmorTrimPattern {
        asset_id: String,
        template_item: String,
        description: TextComponent,
        #[serde_as(as = "BoolFromInt")]
        decal: bool,
    },
    Biome {
        #[serde_as(as = "BoolFromInt")]
        has_precipitation: bool,
        temperature: f32,
        temperature_modifier: Option<String>,
        downfall: f32,
        effects: Effects,
    },
    ChatType {
        chat: Decoration,
        narration: Decoration,
    },
    DamageType {
        message_id: String,
        scaling: String,
        exhaustion: f32,
        effects: Option<String>,
        death_message_type: Option<String>,
    },
    DimensionType {
        fixed_time: Option<i64>,
        #[serde_as(as = "BoolFromInt")]
        has_skylight: bool,
        #[serde_as(as = "BoolFromInt")]
        has_ceiling: bool,
        #[serde_as(as = "BoolFromInt")]
        ultrawarm: bool,
        #[serde_as(as = "BoolFromInt")]
        natural: bool,
        coordinate_scale: f64,
        #[serde_as(as = "BoolFromInt")]
        bed_works: bool,
        #[serde_as(as = "BoolFromInt")]
        respawn_anchor_works: bool,
        min_y: i32,
        height: i32,
        logical_height: i32,
        infiniburn: String,
        effects: String,
        ambient_light: f32,
        #[serde_as(as = "BoolFromInt")]
        piglin_safe: bool,
        #[serde_as(as = "BoolFromInt")]
        has_raids: bool,
        monster_spawn_light_level: IntProvider,
        monster_spawn_block_light_limit: i32,
    },
}
