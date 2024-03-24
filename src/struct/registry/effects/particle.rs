use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Particle {
    options: ParticleOptions,
    probability: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParticleOptions {
    r#type: String,
    value: Option<Value>,
}
