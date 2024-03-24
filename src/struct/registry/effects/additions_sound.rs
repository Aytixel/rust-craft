use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionsSound {
    sound: String,
    tick_chance: f64,
}
