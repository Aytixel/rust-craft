mod dimension_type;
mod recipes;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Datapack {
    dimension_type: HashMap<String, dimension_type::DimensionType>,
    recipes: HashMap<String, recipes::Recipes>,
}

impl Datapack {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            dimension_type: dimension_type::DimensionType::deserialize_folder(
                "./data/minecraft/dimension_type/",
            )?,
            recipes: recipes::Recipes::deserialize_folder("./data/minecraft/recipes/")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        println!("{:#?}", Datapack::new().unwrap());
    }
}
