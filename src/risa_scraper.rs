use crate::{
    FieldTable, form_table::FormTable, horse::Horse, meeting::Meeting, race::Race,
    title_table::TitleTable,
};
use scraper::{Html, Selector};
use std::error;
pub struct RisaScraper;

impl RisaScraper {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn parse_meeting(&self, url: &str) -> Result<Meeting, Box<dyn error::Error>> {
        let body = reqwest::get(url).await?.text().await?;

        let title_raw = body.split("<!-- start of races -->").next().unwrap();

        let (venue, date, meeting_type) = Meeting::parse_meeting_top(title_raw);

        println!("{}", venue);
        println!("{}", date);
        println!("{}", meeting_type);

        let mut races = Vec::new();
        for race_raw in body.split("<!-- start of races -->").skip(1) {
            // Make HTML structure
            let race_html = Html::parse_fragment(&race_raw);
            let race = RisaScraper::parse_race(race_html).unwrap();
            races.push(race);
        }

        Ok(Meeting { races })
    }

    fn parse_race(race_html: Html) -> Result<Race, Box<dyn error::Error>> {
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
