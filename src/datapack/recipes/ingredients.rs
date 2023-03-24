use serde::Deserialize;

use super::result;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Ingredients {
    Ingredient(result::ResultVariant),
    Ingredients(Vec<Ingredients>),
}
