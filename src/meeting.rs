use crate::race::Race;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::error;
use std::fs::File;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meeting {
    pub venue: String,
    pub date: String,
    pub meeting_type: String,
    pub races: Vec<Race>,
}

impl Meeting {
    pub async fn parse_meeting(url: &str) -> Result<Meeting, Box<dyn error::Error>> {
        let body = reqwest::get(url).await?.text().await?;

        let title_raw = body.split("<!-- start of races -->").next().unwrap();

        let (venue, date, meeting_type) = Meeting::parse_meeting_top(title_raw);

        let mut races = Vec::new();
        for race_raw in body.split("<!-- start of races -->").skip(1) {
            // Make HTML structure
            let race_html = Html::parse_fragment(&race_raw);
            let race = Race::parse_race(race_html).unwrap();
            races.push(race);
        }

        Ok(Meeting {
            venue,
            date,
            meeting_type,
            races,
        })
    }

    pub fn parse_meeting_top(title_raw: &str) -> (String, String, String) {
        let title_html = Html::parse_fragment(title_raw);

        // Venue and date from h2 + span
        let h2_selector = Selector::parse(".top > h2").unwrap();
        let h2_elem = title_html.select(&h2_selector).next().unwrap();

        let venue_text = h2_elem
            .text()
            .filter(|t| !t.trim().is_empty())
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();

        let span_selector = Selector::parse("span.race-venue-date").unwrap();
        let date_elem = h2_elem.select(&span_selector).next().unwrap();
        let date_text = date_elem
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        let venue_cleaned = venue_text.replace(&date_text, "").trim().to_string();

        // Meeting type line (under .top)
        let meeting_type_selector = Selector::parse(".top > .meeting-type").unwrap();
        let meeting_type_elem = title_html.select(&meeting_type_selector).next();
        let meeting_type_text = meeting_type_elem
            .map(|e| e.text().collect::<Vec<_>>().join("").trim().to_string())
            .unwrap_or_default();

        // Strip the "Meeting Type: " prefix
        let meeting_type_cleaned = meeting_type_text
            .strip_prefix("Meeting Type:")
            .map(|s| s.trim().to_string())
            .unwrap_or(meeting_type_text);

        (venue_cleaned, date_text, meeting_type_cleaned)
    }

    pub fn save_to_json(&self, path: &str) -> io::Result<()> {
        let json =
            serde_json::to_string_pretty(&self).expect("Failed to serialize Meeting to JSON");
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn from_json(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let meeting =
            serde_json::from_reader(file).expect("Failed to deserialize JSON into Meeting");
        Ok(meeting)
    }
}
