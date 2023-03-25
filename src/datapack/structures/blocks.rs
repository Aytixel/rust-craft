use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Block {
    pos: [i32; 3],
    state: i32,
}
