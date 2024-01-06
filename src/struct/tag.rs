use packet::{DeserializeStruct, SerializeStruct};

#[derive(Debug, Clone, DeserializeStruct, SerializeStruct)]
pub struct Tags {
    registery: String,
    #[variable]
    length: i32,
    #[array(length)]
    array_of_tag: Vec<Tag>,
}

#[derive(Debug, Clone, DeserializeStruct, SerializeStruct)]
pub struct Tag {
    tag_name: String,
    #[variable]
    count: i32,
    #[array(count)]
    #[variable]
    entries: Vec<i32>,
}
