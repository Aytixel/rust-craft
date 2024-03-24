use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ScoreContent {
    pub name: String,
    pub objective: String,
}

impl ScoreContent {
    pub fn new<T: Into<String>>(name: T, objective: T) -> Self {
        Self {
            name: name.into(),
            objective: objective.into(),
        }
    }
}
