use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LightLevelRange {
    max_inclusive: u32,
    min_inclusive: u32,
}

#[derive(Debug, Deserialize)]
pub struct MonsterSpawnLightLevelRange {
    r#type: String,
    value: LightLevelRange,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MonsterSpawnLightLevelVariant {
    MonsterSpawnLightLevel(u32),
    MonsterSpawnLightLevelRange(MonsterSpawnLightLevelRange),
}
