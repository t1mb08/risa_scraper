use crate::{BasicStats, HorseEntry, RaceStart};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FormTableHorse {
    pub horse: HorseEntry,
    pub stats: BasicStats,
    pub starts: Vec<RaceStart>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FormTable {
    pub horses: Vec<FormTableHorse>,
}

impl FormTable {
    pub fn parse_table(
        race_html: &Html,
        num_horse: i32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let form_selector = Selector::parse(".horse-form-table").unwrap();
        let form_tables: Vec<_> = race_html.select(&form_selector).collect();

        assert_eq!(form_tables.len() as i32, num_horse);

        // CSS-valid selector that only gets top-level <tr>s that are odd-numbered
        let row_selector = Selector::parse(":scope > tbody > tr:nth-child(odd)").unwrap();
        let _td_selector = Selector::parse("td").unwrap();

        let mut horses = Vec::new();

        for horse_form in form_tables {
            let parts: Vec<_> = horse_form.select(&row_selector).collect();
            if parts.len() == 3 {
                assert_eq!(parts.len() as i32, 3); // or whatever you expect

                let horse = HorseEntry::parse_horse_from_row(parts[0]);

                let basic_stats = parts[1].text().collect::<String>();
                let stats = BasicStats::parse_basic_stats(&basic_stats);

                let starts = Vec::new();

                let table_selector = Selector::parse(".horse-last-start").unwrap();
                let _tr_selector = Selector::parse("tr").unwrap();

                // Get the table of last starts
                let _last_starts_table = parts[2]
                    .select(&table_selector)
                    .next()
                    .expect("No .horse-last-start table found");

                // Loop through each <tr> row in the table
                // for row in last_starts_table.select(&tr_selector) {
                //     match RaceStart::parse_starts(&row) {
                //         Some(start) => starts.push(start),
                //         None => {}
                //     }
                // }

                horses.push(FormTableHorse {
                    horse,
                    stats,
                    starts,
                });
            } else if parts.len() == 1 {
                let horse = HorseEntry::parse_horse_from_row(parts[0]);

                horses.push(FormTableHorse {
                    horse,
                    stats: BasicStats::default(),
                    starts: Vec::new(),
                });
            }
        }
        Ok(Self { horses })
    }
}
