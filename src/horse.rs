use serde::Serialize;
use std::error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize)]
pub struct Horse {
    pub no: i32,
    pub last10: String,
    pub horse: String,
    pub trainer: String,
    pub jockey: String,
    pub barrier: String,
    pub weight: String,
    pub probable_weight: String,
    pub penalty: String,
    pub hcp_rating: String,
}

impl Horse {
    pub fn save_to_json(horses: &Vec<Horse>, file_path: &str) -> Result<(), Box<dyn error::Error>> {
        let json = serde_json::to_string_pretty(horses)?;
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
