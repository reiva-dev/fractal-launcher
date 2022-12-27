#[derive(Debug, serde::Deserialize)]
pub struct JavaVersion {
    component: String,
    #[serde(rename = "majorVersion")]
    major_version: i32
}