//! Copyright 2022 - Fractal Launcher - ReiRokusanami
//! 
//! This entity that summarizes the `arguments`, `libraries`, and `properties` 
//! required by each version of Minecraft booting.

use std::collections::HashMap;

use super::{Arguments, Asset, Engine, JavaVersion, Library, Logging};

#[derive(Debug, serde::Deserialize)]
pub struct Package {
    arguments: Arguments,
    #[serde(rename = "assetIndex")]
    asset_index: Asset,
    assets: String,
    #[serde(rename = "complianceLevel")]
    compliance_level: i32,
    #[serde(rename = "downloads")]
    engine: HashMap<String, Engine>,
    #[serde(rename = "javaVersion")]
    java_version: JavaVersion,
    libraries: Vec<Library>,
    logging: Logging,

    id: String,
    #[serde(rename = "mainClass")]
    main_class: String,
    #[serde(rename = "minimumLauncherVersion")]
    minimum_launcher_version: i32,
    #[serde(rename = "releaseTime")]
    release_time: String,
    time: String,
    #[serde(rename = "type")]
    package_type: String
}