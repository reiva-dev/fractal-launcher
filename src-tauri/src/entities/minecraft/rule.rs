use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct RuledObject {
    rules: Vec<Rule>,
    value: Option<Valued>
}

#[derive(Debug, serde::Deserialize)]
pub struct Rule {
    action: String,
    features: Option<HashMap<String, bool>>,
    os: Option<OsInfo>
}

#[derive(Debug, serde::Deserialize)]
pub struct OsInfo {
    name: Option<String>,
    version: Option<String>,
    arch: Option<String>
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Valued {
    Single(String),
    Array(Vec<String>)
}