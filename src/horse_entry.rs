use crate::horse_info::HorseInfo;
use scraper::{ElementRef, Selector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HorseEntry {
    pub number: String,
    pub name: String,
    pub silks_img_src: String,
    pub gear: Option<String>,
    pub info: HorseInfo,
}

/// Parses a single horse info row (e.g. parts[0]) into a HorseEntry struct
impl HorseEntry {
    pub fn parse_horse_from_row(horse_info_row: ElementRef) -> HorseEntry {
        let silks_selector = Selector::parse(".Silks").unwrap();
        let silks_img_selector = Selector::parse("img").unwrap();
        let number_selector = Selector::parse(".horse-number").unwrap();
        let name_selector = Selector::parse(".horse-name").unwrap();
        let gear_selector = Selector::parse(".horse-gear").unwrap();
        let plain_selector = Selector::parse(".plain").unwrap();

        let silks_img_src = horse_info_row
            .select(&silks_selector)
            .next()
            .and_then(|silks| silks.select(&silks_img_selector).next())
            .and_then(|img| img.value().attr("src"))
            .unwrap_or("")
            .to_string();

        let number = horse_info_row
            .select(&number_selector)
            .next()
            .map(|n| n.text().collect::<String>())
            .unwrap_or_default();

        let name = horse_info_row
            .select(&name_selector)
            .next()
            .map(|n| n.text().collect::<String>())
            .unwrap_or_default();

        let gear = horse_info_row
            .select(&gear_selector)
            .next()
            .map(|n| n.text().collect::<String>());

        let plain = horse_info_row
            .select(&plain_selector)
            .next()
            .map(|n| n.text().collect::<String>())
            .unwrap_or_default();

        let info = HorseInfo::parse_horse_plain(&plain);

        HorseEntry {
            number,
            name,
            silks_img_src,
            gear,
            info,
        }
    }
}
