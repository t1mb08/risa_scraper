use scraper::{Html, Selector};
use std::error::Error;

pub struct RisaScraper;

impl RisaScraper {
    const URL: &'static str = "https://www.racingaustralia.horse/";

    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_meetings(&self) -> Result<Vec<Meeting>, Box<dyn Error>> {
        let body = reqwest::get(Self::URL).await?.text().await?;
        let doc = Html::parse_document(&body);

        let calendar_selector = Selector::parse(".full_calendar").unwrap();
        let row_selector = Selector::parse(".rows").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let a_selector = Selector::parse("a").unwrap();

        let calendar = doc
            .select(&calendar_selector)
            .next()
            .ok_or("No .full-calendar element found")?;

        let mut meetings = vec![];

        for row in calendar.select(&row_selector) {
            let tds: Vec<_> = row.select(&td_selector).collect();

            if tds.len() != 9 {
                println!("skip header or malformed rows len:{}", tds.len());
                continue; // skip header or malformed rows
            }

            let date = tds[0]
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            for td in &tds[1..] {
                for a in td.select(&a_selector) {
                    let track = a.text().collect::<String>().trim().to_string();
                    let href = a.value().attr("href").unwrap_or("").to_string();
                    let link = format!("https://www.racingaustralia.horse{}", href);

                    meetings.push(Meeting {
                        date: date.clone(),
                        track,
                        link,
                    });
                }
            }
        }

        Ok(meetings)
    }
}

#[derive(Debug)]
pub struct Meeting {
    date: String,
    track: String,
    link: String,
}
