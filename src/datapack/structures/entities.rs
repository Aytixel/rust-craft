use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entitiy {
    pos: [f64; 3],
    #[serde(rename = "blockPos")]
    block_pos: [i32; 3],
}
