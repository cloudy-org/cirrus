use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub version: i8,
    pub dark_mode: bool,
    pub pallet: Pallet
}

#[derive(Deserialize, Serialize)]
pub struct Pallet {
    #[serde(default)]
    primary: Option<String>,
    #[serde(default)]
    surface: Option<String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    accent: Option<String>,
}