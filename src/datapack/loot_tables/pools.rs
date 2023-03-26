mod entries;
mod rolls;

use std::fmt::Debug;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Pool {
    rolls: rolls::RollVariant,
    bonus_roll: Option<f64>,
    conditions: Option<Value>,
    functions: Option<Value>,
    entries: Vec<entries::Entrie>,
}

impl Debug for Pool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Pool");

        s.field("rolls", &self.rolls);

        if let Some(bonus_roll) = &self.bonus_roll {
            s.field("bonus_roll", &bonus_roll);
        }

        if let Some(conditions) = &self.conditions {
            s.field("conditions", &conditions);
        }

        if let Some(functions) = &self.functions {
            s.field("functions", &functions);
        }

        s.field("entries", &self.entries).finish()
    }
}
