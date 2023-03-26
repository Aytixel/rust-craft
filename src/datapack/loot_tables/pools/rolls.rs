use std::fmt::Debug;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Roll {
    r#type: String,
    min: f64,
    max: f64,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RollVariant {
    Roll(f64),
    Rolls(Roll),
}

impl Debug for RollVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RollVariant::Roll(value) => write!(f, "{}", value),
            RollVariant::Rolls(value) => {
                if f.alternate() {
                    write!(f, "{:#?}", value)
                } else {
                    write!(f, "{:?}", value)
                }
            }
        }
    }
}
