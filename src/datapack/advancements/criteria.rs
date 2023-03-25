use std::fmt::Debug;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Criteria {
    conditions: Option<Value>,
    trigger: String,
}

impl Debug for Criteria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Advancement");

        if let Some(conditions) = &self.conditions {
            s.field("conditions", &conditions);
        }

        s.field("trigger", &self.trigger).finish()
    }
}
