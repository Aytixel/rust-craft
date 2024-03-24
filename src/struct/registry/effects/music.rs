use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Music {
    sound: String,
    min_delay: i32,
    max_delay: i32,
    #[serde_as(as = "BoolFromInt")]
    replace_current_music: bool,
}
