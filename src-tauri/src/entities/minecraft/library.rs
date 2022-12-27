use super::Rule;

#[derive(Debug, serde::Deserialize)]
pub struct Library {
    name: String,
    downloads: Artifact,
    rules: Option<Rule>
}

#[derive(Debug, serde::Deserialize)]
pub struct Artifact {
    path: String,
    sha1: String,
    size: i64,
    url: String
}