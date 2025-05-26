use scraper::{Html, Selector};
use std::error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldTableHorse {
    pub scratched: bool,
    pub no: i32,
    pub emergency: bool,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldTable {
    pub horses: Vec<FieldTableHorse>,
}

impl FieldTable {
    pub fn parse_table(race_html: &Html) -> Result<Self, Box<dyn error::Error>> {
        let field_selector = Selector::parse(".race-strip-fields").unwrap();
        let field_table = race_html
            .select(&field_selector)
            .next()
            .ok_or("Missing field table")?;

        let row_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        let mut field_table_rows = field_table.select(&row_selector);

        // Skip header row
        field_table_rows.next();

        let mut horses = Vec::new();

        for row in field_table_rows {
            let mut cells = row.select(&td_selector);
            let scratched = row
                .value()
                .attr("class")
                .map(|class| class.contains("Scratched"))
                .unwrap_or(false);
            let no_text = cells
                .next()
                .and_then(|c| c.text().next())
                .unwrap_or("0")
                .trim()
                .to_lowercase(); // Normalize just in case it's uppercase

            let emergency = no_text.contains('e');
            let no = no_text.replace('e', "").parse::<i32>().unwrap_or(0);
            let last10 = cells
                .next()
                .and_then(|c| c.text().next())
                .unwrap_or("")
                .to_string();
            let horse = cells
                .next()
                .and_then(|c| c.text().next())
                .unwrap_or("")
                .to_string();
            let trainer = cells
                .next()
                .and_then(|c| c.text().next())
                .unwrap_or("")
                .to_string();
            let jockey = cells
                .next()
                .map(|c| c.text().collect::<String>().trim().to_string())
                .unwrap_or_default();
            let barrier = cells
                .next()
                .and_then(|c| c.text().next())
                .unwrap_or("")
                .to_string();
            let weight = cells
                .next()
                .and_then(|c| c.text().next())
                .unwrap_or("")
                .to_string();
            let probable_weight = cells
                .next()
                .and_then(|c| c.text().next())
                .unwrap_or("")
                .to_string();
            let penalty = cells
                .next()
                .and_then(|c| c.text().next())
                .unwrap_or("")
                .to_string();
            let hcp_rating = cells
                .next()
                .and_then(|c| c.text().next())
                .unwrap_or("")
                .to_string();

            horses.push(FieldTableHorse {
                scratched,
                no,
                emergency,
                last10,
                horse,
                trainer,
                jockey,
                barrier,
                weight,
                probable_weight,
                penalty,
                hcp_rating,
            });
        }
        Ok(Self { horses })
    }
}
