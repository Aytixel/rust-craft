use std::fmt::Debug;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Entrie {
    conditions: Option<Value>,
    r#type: String,
    functions: Option<Value>,
    name: Option<String>,
    weight: Option<i32>,
    quality: Option<i32>,
    expand: Option<bool>,
    children: Option<Vec<Entrie>>,
}

impl Debug for Entrie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Entrie");

        if let Some(conditions) = &self.conditions {
            s.field("conditions", &conditions);
        }

        s.field("type", &self.r#type);

        if let Some(functions) = &self.functions {
            s.field("functions", &functions);
        }

        if let Some(name) = &self.name {
            s.field("name", &name);
        }

        if let Some(weight) = &self.weight {
            s.field("weight", &weight);
        }

        if let Some(quality) = &self.quality {
            s.field("quality", &quality);
        }

        if let Some(expand) = &self.expand {
            s.field("expand", &expand);
        }

        if let Some(children) = &self.children {
            s.field("children", &children);
        }

        s.finish()
    }
}
