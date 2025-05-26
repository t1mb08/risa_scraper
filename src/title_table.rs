use regex::Regex;
use scraper::{Html, Selector};

use crate::race_info::RaceInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleTable {
    pub number: u32,
    pub time: String,
    pub name: String,
    pub distance: u32,
    pub info: RaceInfo,
}

impl TitleTable {
    fn extract_main_info(text: &str) -> Option<(u32, String, String, u32)> {
        let re =
            Regex::new(r"Race\s+(\d+)\s*-\s*([0-9:AMP]+)\s+(.*?)\s+\((\d+)\s+METRES\)").ok()?;

        let caps = re.captures(text)?;

        let race_num = caps.get(1)?.as_str().parse().ok()?;
        let time = caps.get(2)?.as_str().to_string();
        let name = caps.get(3)?.as_str().to_string();
        let distance = caps.get(4)?.as_str().parse().ok()?;

        Some((race_num, time, name, distance))
    }

    pub fn parse_table(race_html: &Html) -> Result<Self, Box<dyn std::error::Error>> {
        let title_selector = Selector::parse(".race-title").unwrap();
        let title_table = race_html
            .select(&title_selector)
            .next()
            .ok_or("Missing .race-title table")?;

        let main_selector = Selector::parse("tr > th").unwrap();
        let main = title_table
            .select(&main_selector)
            .next()
            .ok_or("Missing main title row")?
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();

        let (number, time, name, distance) = Self::extract_main_info(&main).unwrap();

        let extra_selector = Selector::parse("tr.race-info > td").unwrap();
        let extra = title_table
            .select(&extra_selector)
            .next()
            .ok_or("Missing extra race info")?;
        let text = extra.text().collect::<Vec<_>>().join(" ");
        let cleaned = text
            .replace('\u{a0}', " ")
            .replace('\n', " ")
            .trim()
            .to_string();

        let info = RaceInfo::parse_race_info(&cleaned);

        Ok(Self {
            number,
            time,
            name,
            distance,
            info,
        })
    }
}
