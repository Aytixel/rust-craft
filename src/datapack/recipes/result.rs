use std::fmt::Debug;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Result {
    count: Option<u32>,
    #[serde(alias = "tag")]
    item: Option<String>,
}

impl Debug for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Result");

        if let Some(count) = &self.count {
            s.field("count", count);
        }

        if let Some(item) = &self.item {
            s.field("item", &item);
        }

        s.finish()
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ResultVariant {
    Item(String),
    Result(Result),
}

impl Debug for ResultVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultVariant::Item(value) => write!(f, "{}", value),
            ResultVariant::Result(value) => {
                if f.alternate() {
                    write!(f, "{:#?}", value)
                } else {
                    write!(f, "{:?}", value)
                }
            }
        }
    }
}
