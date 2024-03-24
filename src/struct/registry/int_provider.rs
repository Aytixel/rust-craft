use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IntProvider {
    Constant(i32),
    Value(IntProviderValue),
    Distribution {
        r#type: String,
        distribution: HashMap<String, IntProviderDistribution>,
    },
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct IntProviderDistribution {
    data: Box<IntProvider>,
    weight: i32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum IntProviderValue {
    #[serde(rename = "minecraft:constant")]
    Constant(IntProviderConstant),
    #[serde(rename = "minecraft:uniform")]
    Uniform {
        min_inclusive: i32,
        max_inclusive: i32,
    },
    #[serde(rename = "minecraft:biased_to_bottom")]
    BiasedToBottom {
        min_inclusive: i32,
        max_inclusive: i32,
    },
    #[serde(rename = "minecraft:clamped")]
    Clamped {
        min_inclusive: i32,
        max_inclusive: i32,
        source: Box<IntProvider>,
    },
    #[serde(rename = "minecraft:clamped_normal")]
    ClampedNormal {
        mean: f32,
        deviation: f32,
        min_inclusive: i32,
        max_inclusive: i32,
    },
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IntProviderConstant {
    Value(i32),
    Map(HashMap<String, i32>),
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use crate::r#struct::{
        IntProvider, IntProviderConstant, IntProviderDistribution, IntProviderValue,
    };

    #[test]
    fn constant_1() {
        assert_eq!(
            serde_json::from_value::<IntProvider>(json!(20)).unwrap(),
            IntProvider::Constant(20)
        );
    }

    #[test]
    fn constant_2() {
        assert_eq!(
            serde_json::from_value::<IntProvider>(json!({
                "type": "minecraft:constant",
                "value": 30
            }))
            .unwrap(),
            IntProvider::Value(IntProviderValue::Constant(IntProviderConstant::Value(30)))
        );
    }

    #[test]
    fn constant_map() {
        assert_eq!(
            serde_json::from_value::<IntProvider>(json!({
                "type": "minecraft:constant",
                "value": {
                    "test": 30,
                    "test2": 50,
                }
            }))
            .unwrap(),
            IntProvider::Value(IntProviderValue::Constant(IntProviderConstant::Map(
                HashMap::from([("test".to_string(), 30), ("test2".to_string(), 50),]),
            )))
        );
    }

    #[test]
    fn uniform() {
        assert_eq!(
            serde_json::from_value::<IntProvider>(json!({
                "type": "minecraft:uniform",
                "value": {
                    "min_inclusive": 30,
                    "max_inclusive": 50,
                }
            }))
            .unwrap(),
            IntProvider::Value(IntProviderValue::Uniform {
                min_inclusive: 30,
                max_inclusive: 50
            })
        );
    }

    #[test]
    fn biased_to_bottom() {
        assert_eq!(
            serde_json::from_value::<IntProvider>(json!({
                "type": "minecraft:biased_to_bottom",
                "value": {
                    "min_inclusive": 30,
                    "max_inclusive": 50,
                }
            }))
            .unwrap(),
            IntProvider::Value(IntProviderValue::BiasedToBottom {
                min_inclusive: 30,
                max_inclusive: 50
            })
        );
    }

    #[test]
    fn clamped() {
        assert_eq!(
            serde_json::from_value::<IntProvider>(json!({
                "type": "minecraft:clamped",
                "value": {
                    "min_inclusive": 30,
                    "max_inclusive": 50,
                    "source": {
                        "type": "minecraft:constant",
                        "value": 10
                    }
                }
            }))
            .unwrap(),
            IntProvider::Value(IntProviderValue::Clamped {
                min_inclusive: 30,
                max_inclusive: 50,
                source: Box::new(IntProvider::Value(IntProviderValue::Constant(
                    IntProviderConstant::Value(10)
                ))),
            })
        );
    }

    #[test]
    fn clamped_normal() {
        assert_eq!(
            serde_json::from_value::<IntProvider>(json!({
                "type": "minecraft:clamped_normal",
                "value": {
                    "mean": 0.1,
                    "deviation": 1.0,
                    "min_inclusive": 30,
                    "max_inclusive": 50,
                }
            }))
            .unwrap(),
            IntProvider::Value(IntProviderValue::ClampedNormal {
                mean: 0.1,
                deviation: 1.0,
                min_inclusive: 30,
                max_inclusive: 50,
            })
        );
    }

    #[test]
    fn distribution() {
        assert_eq!(
            serde_json::from_value::<IntProvider>(json!({
                "type": "minecraft:weighted_list",
                "distribution": {
                    "test": {
                        "data": {
                            "type": "minecraft:constant",
                            "value": 10,
                        },
                        "weight": 32
                    }
                }
            }))
            .unwrap(),
            IntProvider::Distribution {
                r#type: "minecraft:weighted_list".to_string(),
                distribution: HashMap::from([(
                    "test".to_string(),
                    IntProviderDistribution {
                        data: Box::new(IntProvider::Value(IntProviderValue::Constant(
                            IntProviderConstant::Value(10)
                        ))),
                        weight: 32,
                    }
                )]),
            }
        );
    }
}
