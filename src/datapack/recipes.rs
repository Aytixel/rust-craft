mod ingredients;

use std::collections::HashMap;
use std::fmt::Debug;

use datapack_macro::DeserializeFolder;
use serde::Deserialize;

use self::ingredients::Ingredients;

use super::items;

#[derive(Deserialize, DeserializeFolder)]
pub struct Recipes {
    r#type: String,
    category: Option<String>,
    group: Option<String>,
    ingredients: Option<Ingredients>,
    key: Option<HashMap<String, Ingredients>>,
    pattern: Option<Vec<String>>,
    result: Option<items::ItemVariant>,
    experience: Option<f64>,
    cookingtime: Option<u32>,
    show_notification: Option<bool>,
}

impl Debug for Recipes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Recipes");

        s.field("type", &self.r#type);

        if let Some(category) = &self.category {
            s.field("category", category);
        }

        if let Some(group) = &self.group {
            s.field("group", &group);
        }

        if let Some(ingredients) = &self.ingredients {
            s.field("ingredients", &ingredients);
        }

        if let Some(key) = &self.key {
            s.field("key", &key);
        }

        if let Some(pattern) = &self.pattern {
            s.field("pattern", &pattern);
        }

        if let Some(result) = &self.result {
            s.field("result", &result);
        }

        if let Some(experience) = &self.experience {
            s.field("experience", &experience);
        }

        if let Some(cookingtime) = &self.cookingtime {
            s.field("cookingtime", &cookingtime);
        }

        if let Some(show_notification) = &self.show_notification {
            s.field("show_notification", &show_notification);
        }

        s.finish()
    }
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
