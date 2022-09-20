use std::collections::HashMap;

use crate::field::{Field, Fields};

use super::byte_order::{
    ByteOrder
};
use serde::{
    Deserialize, Deserializer,
};
use toml::{Value, value::Table};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Layer {
    #[serde(default)]
    pub name:           String,
    #[serde(default)]
    pub size:           usize,
    #[serde(default)]
    pub byte_order:     ByteOrder,
    #[serde(default)]
    pub next_layer:     String,
    #[serde(flatten)]
//    #[serde(deserialize_with = "Field::from_table")]
    pub fields:         HashMap<String, Field>,
}

impl Layer {
    pub fn from_value<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Layer = Deserialize::deserialize(deserializer)?;
        Ok(s)
    }

    pub fn from_table<'de, D>(deserializer: D) -> Result<Vec<Self>, D::Error> 
    where
        D: Deserializer<'de>,
    {
        let layers: Table = Deserialize::deserialize(deserializer)?;
        println!("Layers: {:?}", layers);
        Ok(layers.into_iter().map(|x| {
            let mut l: Layer = x.1.try_into().unwrap();
            println!("Layer: {:?}", l);
            if l.name == "" {
                l.name = x.0;
            }
            
            l
        }).collect()
        )
    }

}


