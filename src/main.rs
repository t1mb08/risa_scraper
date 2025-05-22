use risa_scaper::*;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;
use std::error;
use std::fs::File;
use std::io::prelude::*;

const URL: &str = "https://www.racingaustralia.horse/FreeFields/Form.aspx?Key=2025May23%2CQLD%2CAquis%20Park%20Gold%20Coast";

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, world!");
    let body = reqwest::get(URL).await?.text().await?;

    let races_split: Vec<&str> = body.split("<!-- start of races -->").collect();

    let race_1 = races_split[1];
    // Write file for checking
    let mut file = File::create("r1.html")?;
    file.write_all(race_1.as_bytes())?;

    // Make HTML structure
    let r1_html = Html::parse_fragment(&race_1);
    let r1_horses = RisaScraper::parse_race(r1_html).unwrap();
    Ok(())
}

struct RisaScraper;

impl RisaScraper {
    fn parse_meeting() {
        todo!();
    }

    fn parse_race(race_html: Html) -> Result<(), Box<dyn error::Error>> {
        // let title =
        let horses = Self::parse_risa_field_table(&race_html).unwrap();
        let forms = Self::parse_risa_form_table(race_html, horses.len() as i32).unwrap();
        Ok(())
    }

    fn parse_risa_title_table(table: Html) -> HashMap<String, String> {
        todo!();
    }

    fn parse_risa_field_table(race_html: &Html) -> Result<Vec<Horse>, Box<dyn error::Error>> {
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
            let no = cells
                .next()
                .and_then(|c| c.text().next())
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0);
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
                .and_then(|c| c.text().next())
                .unwrap_or("")
                .to_string();
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

            horses.push(Horse {
                no,
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
        Ok(horses)
    }

    fn parse_risa_form_table(
        race_html: Html,
        num_horse: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let form_selector = Selector::parse(".horse-form-table").unwrap();
        let form_tables: Vec<_> = race_html.select(&form_selector).collect();

        assert_eq!(form_tables.len() as i32, num_horse);

        // CSS-valid selector that only gets top-level <tr>s that are odd-numbered
        let row_selector = Selector::parse(":scope > tbody > tr:nth-child(odd)").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        for horse_form in form_tables {
            let parts: Vec<_> = horse_form.select(&row_selector).collect();
            assert_eq!(parts.len() as i32, 3); // or whatever you expect

            let horse_entry = HorseEntry::parse_horse_from_row(parts[0]);
            println!("{:#?}", horse_entry);

            let basic_stats = parts[1].text().collect::<String>();
            let stats = BasicStats::parse_basic_stats(&basic_stats);
            println!("{:#?}", stats);

            let mut race_starts = Vec::new();

            let table_selector = Selector::parse(".horse-last-start").unwrap();
            let tr_selector = Selector::parse("tr").unwrap();

            // Get the table of last starts
            let last_starts_table = parts[2]
                .select(&table_selector)
                .next()
                .expect("No .horse-last-start table found");

            // Loop through each <tr> row in the table
            for row in last_starts_table.select(&tr_selector) {
                match RaceStart::parse_starts(&row) {
                    Some(start) => race_starts.push(start),
                    None => {
                        eprintln!("Failed to parse line: {:#?}", row);
                        // Optionally continue or break
                        // break;
                    }
                }
                // break;
            }

            for race in race_starts {
                println!("{:#?}", race);
            }

            break;
        }

        Ok(())
    }
}
