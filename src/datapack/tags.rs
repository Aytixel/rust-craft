use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tag {
    expected: bool,
    id: String,
}

#[derive(Debug, Deserialize)]
pub struct Tags {
    tags: Vec<Tag>,
}
