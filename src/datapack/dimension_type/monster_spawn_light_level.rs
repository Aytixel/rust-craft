use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Value {
    max_inclusive: u32,
    min_inclusive: u32,
}

#[derive(Debug, Deserialize)]
pub struct Struct {
    #[serde(rename = "type")]
    _type: String,
    value: Value,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MonsterSpawnLightLevel {
    U32(u32),
    Struct(Struct),
}
