use std::fmt::Debug;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Item {
    count: Option<u32>,
    #[serde(alias = "tag")]
    item: Option<String>,
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Item");

        if let Some(count) = &self.count {
            s.field("count", count);
        }

        if let Some(item) = &self.item {
            s.field("item", &item);
        }

        s.finish()
    }
}

#[derive(Debug, Deserialize)]
pub struct Items {
    items: Vec<ItemVariant>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ItemVariant {
    Items(Items),
    ItemString(String),
    Item(Item),
}

impl Debug for ItemVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemVariant::Items(value) => {
                if f.alternate() {
                    write!(f, "{:#?}", value)
                } else {
                    write!(f, "{:?}", value)
                }
            }
            ItemVariant::ItemString(value) => write!(f, "{}", value),
            ItemVariant::Item(value) => {
                if f.alternate() {
                    write!(f, "{:#?}", value)
                } else {
                    write!(f, "{:?}", value)
                }
            }
        }
    }
}
