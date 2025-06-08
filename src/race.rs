use crate::{FieldTable, form_table::FormTable, horse::Horse, title_table::TitleTable};
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Race {
    #[serde(flatten)]
    pub title: TitleTable,
    pub field: Vec<Horse>,
}

impl Race {
    pub fn parse_race(race_html: Html) -> Result<Race, Box<dyn error::Error>> {
        let title = TitleTable::parse_table(&race_html).unwrap();

        println!("{}", title.name);
        let field = FieldTable::parse_table(&race_html).unwrap();
        let form = FormTable::parse_table(&race_html, field.horses.len() as i32).unwrap();

        let r1 = Race {
            title,
            field: Horse::vec_zip(field.horses, form.horses),
        };
        Ok(r1)
    }
}
