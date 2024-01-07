use serde::Deserialize;

/// Configuration
#[derive(Deserialize, Debug)]
pub struct Config {
    pub data: SinnerData,
    pub input_sinner_folder: String,
    pub output_sinner_folder: String,
    pub asset_folder: String,
}

/// Each sinner
#[derive(Deserialize, Debug)]
pub struct SinnerData {
    pub sinner: Vec<Sinner>,
}

/// Each sinner's array of identities
#[derive(Deserialize, Debug)]
pub struct Sinner {
    pub id: Vec<Identity>,
    pub name: String,
    pub path: String,
}

/// Each identity's name and image link
#[derive(Deserialize, Debug)]
pub struct Identity {
    pub name: String,
    pub rarity: u8,
    pub image: String,
}
