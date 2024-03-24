use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AmbientSound {
    SoundTrack(String),
    AmbientSound {
        sound_id: String,
        range: Option<f32>,
    },
}
