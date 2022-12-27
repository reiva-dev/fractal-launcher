#[derive(Debug, serde::Deserialize)]
pub struct Asset {
    id: String,
    sha1: String,
    size: i64,
    #[serde(rename = "totalSize")]
    total_size: i64,
    url: String
}