use std::fmt::Debug;

use serde::Deserialize;

use super::item;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Ingredients {
    Ingredient(item::ItemVariant),
    Ingredients(Vec<Ingredients>),
}

impl Debug for Ingredients {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ingredients::Ingredient(value) => {
                if f.alternate() {
                    write!(f, "{:#?}", value)
                } else {
                    write!(f, "{:?}", value)
                }
            }
            Ingredients::Ingredients(value) => {
                if f.alternate() {
                    write!(f, "{:#?}", value)
                } else {
                    write!(f, "{:?}", value)
                }
            }
        }
    }
}
