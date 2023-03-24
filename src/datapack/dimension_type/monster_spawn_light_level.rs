use std::fmt::Debug;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LightLevelRange {
    max_inclusive: u32,
    min_inclusive: u32,
}

#[derive(Deserialize)]
pub struct MonsterSpawnLightLevelRange {
    r#type: String,
    value: LightLevelRange,
}

impl Debug for MonsterSpawnLightLevelRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MonsterSpawnLightLevelRange")
            .field("type", &self.r#type)
            .field("value", &self.value)
            .finish()
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum MonsterSpawnLightLevelVariant {
    MonsterSpawnLightLevel(u32),
    MonsterSpawnLightLevelRange(MonsterSpawnLightLevelRange),
}

impl Debug for MonsterSpawnLightLevelVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonsterSpawnLightLevelVariant::MonsterSpawnLightLevel(value) => write!(f, "{}", value),
            MonsterSpawnLightLevelVariant::MonsterSpawnLightLevelRange(value) => {
                if f.alternate() {
                    write!(f, "{:#?}", value)
                } else {
                    write!(f, "{:?}", value)
                }
            }
        }
    }
}
