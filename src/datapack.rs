mod dimension_type;

use std::collections::HashMap;

pub trait DeserializeFolder {
    fn deserialize_folder() -> Result<HashMap<String, Self>, String>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Datapack {
    dimension_type: HashMap<String, dimension_type::DimensionType>,
}

impl Datapack {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            dimension_type: dimension_type::DimensionType::deserialize_folder()?,
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
