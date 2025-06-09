use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct PrizeMoney {
    pub total: u32,
    pub places: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RaceInfo {
    pub prizemoney: Option<PrizeMoney>,
    pub animal_deduction: Option<String>,
    pub insurance_deduction: Option<String>,
    pub race_conditions: Option<String>,
    pub min_weight: Option<String>,
    pub weight_raised: Option<String>,
    pub age: Option<String>,
    pub apprentice_claim: bool,
    pub track_name: Option<String>,
    pub track_type: Option<String>,
    pub field_limit: Option<u8>,
    pub emergencies: Option<u8>,
}

impl RaceInfo {
    pub fn extract_main_info(text: &str) -> Option<(u32, String, String, u32)> {
        let re =
            Regex::new(r"Race\s+(\d+)\s*-\s*([0-9:AMP]+)\s+(.*?)\s+\((\d+)\s+METRES\)").ok()?;
        let caps = re.captures(text)?;
        let race_num = caps.get(1)?.as_str().parse().ok()?;
        let time = caps.get(2)?.as_str().to_string();
        let name = caps.get(3)?.as_str().to_string();
        let distance = caps.get(4)?.as_str().parse().ok()?;
        Some((race_num, time, name, distance))
    }

    pub fn parse_race_info(text: &str) -> Self {
        let num = |s: &str| s.replace(",", "").parse::<u32>().ok();

        let total = Regex::new(r"Of\s+\$(\d[\d,]*)")
            .ok()
            .and_then(|r| r.captures(text).and_then(|c| num(&c[1])));

        let place_re = Regex::new(r"(\d+(?:st|nd|rd|th))\s*\$\s*([\d,]+)").unwrap();
        let places = place_re
            .captures_iter(text)
            .filter_map(|cap| Some((cap[1].to_string(), num(&cap[2])?)))
            .collect::<HashMap<_, _>>();

        let prizemoney = total.map(|t| PrizeMoney { total: t, places });

        let animal_deduction = Regex::new(r"Animal Care.*?rate of (\d+%)")
            .ok()
            .and_then(|r| r.captures(text).map(|c| c[1].to_string()));

        let insurance_deduction = Regex::new(r"Insurance.*?rate of (\d+%)")
            .ok()
            .and_then(|r| r.captures(text).map(|c| c[1].to_string()));

        // New: Extract race conditions line
        let race_conditions = Regex::new(
    r"(BenchMark \d+|Handicap|No class restriction|Set Weights|Weight for Age|Quality|Maiden)"
)
.ok()
.and_then(|r| r.captures(text).and_then(|c| c.get(1).map(|m| m.as_str().to_string())));

        let min_weight = Regex::new(r"Minimum Weight (\d+(?:\.\d+)?kg)")
            .ok()
            .and_then(|r| r.captures(text).map(|c| c[1].to_string()));

        let weight_raised = Regex::new(r"Weight Raised (\d+(?:\.\d+)?kg)")
            .ok()
            .and_then(|r| r.captures(text).map(|c| c[1].to_string()));

        let age = normalize_age_label(text);

        let apprentice_claim = text.contains("Apprentices can claim");

        let track_info = Regex::new(r"Track Name:\s*(\w+).*?Track Type:\s*(\w+)")
            .ok()
            .and_then(|r| r.captures(text));

        let field_info = Regex::new(r"Field Limit:\s*(\d+)(?:\s*\+\s*(\d+)\s*EM)?")
            .ok()
            .and_then(|r| r.captures(text));

        RaceInfo {
            prizemoney,
            animal_deduction,
            insurance_deduction,
            race_conditions,
            min_weight,
            weight_raised,
            age,
            apprentice_claim,
            track_name: track_info.as_ref().map(|c| c[1].to_string()),
            track_type: track_info.as_ref().map(|c| c[2].to_string()),
            field_limit: field_info.as_ref().and_then(|c| c[1].parse().ok()),
            emergencies: field_info
                .as_ref()
                .and_then(|c| c.get(2).and_then(|m| m.as_str().parse().ok())),
        }
    }
}

fn normalize_age_label(text: &str) -> Option<String> {
    let re = Regex::new(r"(?i)\b(One|Two|Three|Four|Five|Six|Seven|Eight|Nine|Ten|Eleven|Twelve|Thirteen|Fourteen|Fifteen|Sixteen|Seventeen|Eighteen|Nineteen|Twenty|\d+)-Years?-Old\b").unwrap();

    let word_to_num: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
        ("eleven", 11),
        ("twelve", 12),
        ("thirteen", 13),
        ("fourteen", 14),
        ("fifteen", 15),
        ("sixteen", 16),
        ("seventeen", 17),
        ("eighteen", 18),
        ("nineteen", 19),
        ("twenty", 20),
    ]
    .iter()
    .cloned()
    .collect();

    let mut ages: Vec<u32> = re
        .captures_iter(text)
        .filter_map(|cap| {
            let val = &cap[1];
            val.parse::<u32>()
                .ok()
                .or_else(|| word_to_num.get(&val.to_lowercase() as &str).copied())
        })
        .collect();

    if ages.is_empty() {
        return None;
    }

    ages.sort();
    ages.dedup();

    let upwards = text.to_lowercase().contains("upwards");

    let label = if ages.len() == 1 {
        format!("{}yo{}", ages[0], if upwards { "+" } else { "" })
    } else {
        let joined = ages
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("&");
        format!("{}yo", joined)
    };

    Some(label)
}
