use std::fmt::Debug;

use datapack_macro::DeserializeFolder;
use serde::Deserialize;

#[derive(Deserialize, DeserializeFolder)]
pub struct DamageType {
    effects: Option<String>,
    exhaustion: f64,
    message_id: String,
    scaling: String,
}

impl Debug for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("DamageType");

        if let Some(effects) = &self.effects {
            s.field("effects", &effects);
        }

        s.field("exhaustion", &self.exhaustion)
            .field("message_id", &self.message_id)
            .field("scaling", &self.scaling)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_folder() {
        println!(
            "{:#?}",
            DamageType::deserialize_folder("./data/minecraft/damage_type/").unwrap()
        );
    }
}
