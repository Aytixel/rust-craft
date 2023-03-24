mod ingredients;
mod result;

use std::collections::HashMap;

use datapack_macro::DeserializeFolder;
use serde::Deserialize;

use self::ingredients::Ingredients;

#[derive(Debug, Deserialize, DeserializeFolder)]
pub struct Recipes {
    r#type: String,
    category: Option<String>,
    group: Option<String>,
    ingredients: Option<Ingredients>,
    key: Option<HashMap<String, Ingredients>>,
    pattern: Option<Vec<String>>,
    result: Option<result::ResultVariant>,
    experience: Option<f64>,
    cookingtime: Option<u32>,
    show_notification: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_folder() {
        println!(
            "{:#?}",
            Recipes::deserialize_folder("./data/minecraft/recipes/").unwrap()
        );
    }
}
