//! Copyright 2022 - Fractal Launcher - ReiRokusanami
//! 
//! This Entity that represents the main body of the game.
//! It is used to deserialize information such as Client Mapping.

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Engine {
    sha1: String,
    size: i64,
    url: String
}