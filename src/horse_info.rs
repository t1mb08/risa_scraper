use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HorseInfo {
    pub age: String,
    pub colour: String,
    pub sex: String,
    pub foaled: String,
    pub sire: String,
    pub dam: String,
    pub dam_sire: String,
    pub breeder: String,
    pub owners: String,
    pub colours: String,
}

impl HorseInfo {
    pub fn parse_horse_plain(plain: &str) -> HorseInfo {
        let full_regex = Regex::new(
            r"(?x)
        (\d+)\s+year\s+old\s+         # age
        (\w+)\s+                      # colour
        (\w+)\s+                      # sex
        \(([\d-]+)\)\s+              # foaled
        Sire:\s*(.+?)\s+             # sire (non-greedy up to next keyword)
        Dam:\s*([^(]+)\(([^)]+)\).*? # dam and dam sire
        Breeder:\s*(.+)\s+
        Owners:\s*(.+)\s+
        Colours:\s*(.+)
    ",
        )
        .unwrap();
        if let Some(caps) = full_regex.captures(plain) {
            let age = caps.get(1).unwrap().as_str().to_string();
            let colour = caps.get(2).unwrap().as_str().to_string();
            let sex = caps.get(3).unwrap().as_str().to_string();
            let foaled = caps.get(4).unwrap().as_str().to_string();
            let sire = caps.get(5).unwrap().as_str().trim().to_string();
            let dam = caps.get(6).unwrap().as_str().trim().to_string();
            let dam_sire = caps.get(7).unwrap().as_str().trim().to_string();
            let breeder = caps.get(8).unwrap().as_str().trim().to_string();
            let owners = caps.get(9).unwrap().as_str().trim().to_string();
            let colours = caps.get(10).unwrap().as_str().trim().to_string();

            return HorseInfo {
                age,
                colour,
                sex,
                foaled,
                sire,
                dam,
                dam_sire,
                breeder,
                owners,
                colours,
            };
        } else {
            // (same regex code as before)
            let age_regex = Regex::new(r"(\d+)\s+year old\s+(\w+)\s+(\w+)\s+\(([\d-]+)\)").unwrap();
            let sire_regex = Regex::new(r"Sire:\s*([^\n\r]+)").unwrap();
            let dam_regex = Regex::new(r"Dam:\s*([^(]+)").unwrap();
            let dam_sire_regex = Regex::new(r"\(([^)]+)\)").unwrap();
            let breeder_regex = Regex::new(r"Breeder:\s*([^\n\r]+)").unwrap();
            let owners_regex = Regex::new(r"Owners:\s*([^\n\r]+)").unwrap();
            let colours_regex = Regex::new(r"Colours:\s*([^\n\r]+)").unwrap();

            let age_caps = age_regex.captures(plain);
            let sire_caps = sire_regex.captures(plain);
            let dam_caps = dam_regex.captures(plain);
            let dam_sire_caps = dam_sire_regex.captures(plain);
            let breeder_caps = breeder_regex.captures(plain);
            let owners_caps = owners_regex.captures(plain);
            let colours_caps = colours_regex.captures(plain);

            let age = age_caps
                .as_ref()
                .and_then(|c| c.get(1))
                .map(|m| m.as_str())
                .unwrap_or("")
                .to_string();
            let colour = age_caps
                .as_ref()
                .and_then(|c| c.get(2))
                .map(|m| m.as_str())
                .unwrap_or("")
                .to_string();
            let sex = age_caps
                .as_ref()
                .and_then(|c| c.get(3))
                .map(|m| m.as_str())
                .unwrap_or("")
                .to_string();
            let foaled = age_caps
                .as_ref()
                .and_then(|c| c.get(4))
                .map(|m| m.as_str())
                .unwrap_or("")
                .to_string();
            let sire = sire_caps
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim())
                .unwrap_or("")
                .split(" Dam:")
                .next()
                .unwrap()
                .to_string();
            let dam = dam_caps
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim())
                .unwrap_or("")
                .to_string();
            let dam_sire = dam_sire_caps
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim())
                .unwrap_or("")
                .to_string();
            let breeder = breeder_caps
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim())
                .unwrap_or("")
                .to_string();
            let owners = owners_caps
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim())
                .unwrap_or("")
                .to_string();
            let colours = colours_caps
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim())
                .unwrap_or("")
                .to_string();

            HorseInfo {
                age,
                colour,
                sex,
                foaled,
                sire,
                dam,
                dam_sire,
                breeder,
                owners,
                colours,
            }
        }
    }
}
