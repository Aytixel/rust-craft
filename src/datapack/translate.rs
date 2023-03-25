use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Translate {
    translate: String,
}
