use std::iter::FromIterator;

use serde::{Deserialize, Deserializer};
use toml::value::Table;



#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Field {
    #[serde(default)]
    pub index:          usize,
    #[serde(default)]
    pub name:           String,
    #[serde(default)]
    pub field_type:     FieldType,
    #[serde(default)]
    pub value_type:     ValueType,
    #[serde(flatten)]
    #[serde(default)]
    pub value:          Option<Value>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Fields(pub Vec<Field>);

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub enum FieldType {
    #[serde(alias="fixed")]
    Fixed,
    #[serde(alias="layer_state", alias="layer-state", alias="state")]
    LayerState,
    #[serde(alias="layer_size", alias="layer-size", alias="size")]
    LayerSize,
    #[serde(alias="variable", alias="var")]
    Variable,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub enum ValueType {
    #[serde(alias="none")]
    None, 
    #[serde(alias="u8")]
    U8,
    #[serde(alias="u16")]
    U16,
    #[serde(alias="u32")]
    U32,
    #[serde(alias="u64")]
    U64,
    #[serde(alias="i8")]
    I8,
    #[serde(alias="i16")]
    I16,
    #[serde(alias="i32")]
    I32,
    #[serde(alias="i64")]
    I64,
    #[serde(alias="f32", alias = "float")]
    Float,
    #[serde(alias="f64", alias = "double")]
    Double,
    #[serde(alias="bit")]
    Bit,
    #[serde(alias="bytes")]
    Bytes,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub enum Value {
    None,
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Float(f32),
    Double(f64),
    Bit(u8),
    Bytes(Vec<u8>),
}

impl Field {
    pub fn from_table<'de, D>(deserializer: D) -> Result<Vec<Self>, D::Error> 
    where
        D: Deserializer<'de>,
    {
        let fields: Table = Deserialize::deserialize(deserializer)?;
        
        Ok(fields.into_iter().map(|x| {
            println!("\n{:?}", x);
            let mut f: Field = x.1.try_into().unwrap();
            /*if f.name == "" {
                f.name = x.0;
            }*/
            f
        }).collect()
        )
    }
}

impl Fields {
    
    pub fn from_table<'de, D>(deserializer: D) -> Result<Self, D::Error> 
    where
        D: Deserializer<'de>,
    {
        let fields: Table = Deserialize::deserialize(deserializer)?;
        
        Ok(fields.into_iter().map(|x| {
            let mut f: Field = x.1.try_into().unwrap();
            if f.name == "" {
                f.name = x.0;
            }
            f
        }).collect()
        )
    }
}

impl FromIterator<Field> for Fields {
    fn from_iter<T: IntoIterator<Item = Field>>(iter: T) -> Self {
        let mut v = Self { 0: Vec::new()};
        for f in iter {
            v.0.push(f);
        }
        v
    }
}

impl Default for FieldType {
    fn default() -> Self {
        Self::Variable
    }
}

impl Default for ValueType {
    fn default() -> Self {
        Self::None
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::None
    }
}

