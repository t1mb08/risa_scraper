use crate::{FieldTable, form_table::FormTable, horse::Horse, title_table::TitleTable};
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::error;

use crate::race_info::RaceInfo;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Race {
    pub number: u32,
    pub time: String,
    pub name: String,
    pub distance: u32,
    pub info: RaceInfo,
    pub field: Vec<Horse>,
}

impl Race {
    pub fn parse_race(race_html: Html) -> Result<Race, Box<dyn error::Error>> {
        let title = TitleTable::parse_table(&race_html).unwrap();

        println!("{}", title.name);
        let field = FieldTable::parse_table(&race_html).unwrap();
        let form = FormTable::parse_table(&race_html, field.horses.len() as i32).unwrap();

        let r1 = Race {
            number: title.number,
            time: title.time,
            name: title.name,
            distance: title.distance,
            info: title.info,
            field: Horse::vec_zip(field.horses, form.horses),
        };
        Ok(r1)
    }
}
