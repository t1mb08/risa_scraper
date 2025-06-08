use std::error::Error;

use risa_scraper::risa_scraper::RisaScraper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello Scraper!!!");

    let risa_scraper = RisaScraper::new();

    let meetings = risa_scraper.get_meetings().await?;
    println!("{:#?}", meetings);
    Ok(())
}
