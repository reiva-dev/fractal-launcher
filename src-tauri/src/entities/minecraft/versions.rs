#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VersionManifest {
    latest: Latest,
    versions: Vec<Version>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Latest {
    release: String,
    snapshot: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Version {
    id: String,
    #[serde(rename = "type")]
    vc_type: String,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String
}

impl VersionManifest {
    pub fn versions(&self) -> &[Version] {
        &self.versions
    }
}

impl Version {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn vc_type(&self) -> &str {
        &self.vc_type
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn release_time(&self) -> &str {
        &self.release_time
    }
}