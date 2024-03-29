use std::collections::HashMap;

use super::rule::RuledObject;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Arguments {
    game: Vec<BootArg>,
    jvm: Vec<Jvm>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BootArg {
    String(String),
    Object(RuledObject)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Jvm {
    String(String),
    Object(RuledObject)
}