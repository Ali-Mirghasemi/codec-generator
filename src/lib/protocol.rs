use std::{iter::Map, collections::HashMap};

use crate::layer::Layer;

use super::byte_order::{
    ByteOrder
};
use serde::{
    Deserialize,
};

use toml::{Value, value::{Table}};

#[derive(Debug, Clone, Deserialize)]
pub struct Protocol {
    pub name:               String,
    pub style:              SyntaxStyle,
    #[serde(default)]
    pub version_code:       u32,
    #[serde(default)]
    pub version:            String,
    #[serde(flatten)]
    //#[serde(deserialize_with = "Layer::from_table")]
    pub layers:             HashMap<String, Layer>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum SyntaxStyle {
    #[serde(alias = "sneak_case")]
    SneakCase,
    CamelCase,
    #[serde(alias = "pascalCase")]
    PascalCase,
}
