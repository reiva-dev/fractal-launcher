use std::collections::HashMap;

use super::Rule;

#[derive(Debug, serde::Deserialize)]
pub struct Library {
    name: String,
    downloads: Target,
    extract: Option<Extraction>,
    natives: Option<HashMap<String, String>>,
    rules: Option<Vec<Rule>>
}

#[derive(Debug, serde::Deserialize)]
pub struct Target {
    artifact: Option<Artifact>,
    classifiers: Option<HashMap<String, Artifact>>
}

#[derive(Debug, serde::Deserialize)]
pub struct Artifact {
    path: String,
    sha1: String,
    size: i64,
    url: String
}

#[derive(Debug, serde::Deserialize)]
pub struct Extraction {
    exclude: Vec<String>
}