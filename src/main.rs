use risa_scaper::risa_scraper::RisaScraper;
use std::error;

const URL: &str =
    "https://www.racingaustralia.horse/FreeFields/Form.aspx?Key=2025May28%2CQLD%2CDoomben";

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let risa = RisaScraper::new();
    let town = risa.parse_meeting(URL).await?;
    println!("{:#?}", town);
    Ok(())
}
