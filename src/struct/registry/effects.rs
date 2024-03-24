mod additions_sound;
mod ambient_sound;
mod mood_sound;
mod music;
mod particle;

use serde::{Deserialize, Serialize};

pub use additions_sound::*;
pub use ambient_sound::*;
pub use mood_sound::*;
pub use music::*;
pub use particle::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Effects {
    fog_color: i32,
    water_color: i32,
    water_fog_color: i32,
    sky_color: i32,
    foliage_color: Option<i32>,
    grass_color: Option<i32>,
    grass_color_modifier: Option<String>,
    particle: Option<Particle>,
    ambient_sound: Option<AmbientSound>,
    mood_sound: Option<MoodSound>,
    additions_sound: Option<AdditionsSound>,
    music: Option<Music>,
}
