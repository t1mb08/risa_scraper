use regex::Regex;
use scraper::{ElementRef, Html, Selector};

#[derive(Debug)]
pub struct RaceStart {
    pub pos: String,
    pub field_size: String,
    pub track: String,
    pub date: String,
    pub distance: String,
    pub condition: String,
    pub race_class: String,
    pub prizemoney: String,
    pub placing_prizemoney: Option<String>,
    pub jockey: String,
    pub apprentice: Option<String>,
    pub weight: String,
    pub weight_claimed: Option<String>,
    pub barrier: String,
    pub first: Option<String>,
    pub second: Option<String>,
    pub third: Option<String>,
    pub time: String,
    pub sectional: Option<String>,
    pub margin: String,
    pub pos_800: Option<String>,
    pub pos_400: Option<String>,
    pub odds: Option<String>,
}

impl RaceStart {
    pub fn parse_starts(row: &ElementRef) -> Option<RaceStart> {
        // Extract Position
        let pos_selector = Selector::parse(".Pos").ok()?;

        let pos_raw = row
            .select(&pos_selector)
            .next()?
            .text()
            .collect::<String>()
            .trim()
            .to_string();
        let pos_re = Regex::new(r"(?P<pos>\d+)\u{a0}of\u{a0}(?P<field_size>\d+)").unwrap();

        let caps = pos_re.captures(&pos_raw)?;

        let pos = caps.name("pos")?.as_str().to_string();
        let field_size = caps.name("field_size")?.as_str().to_string();

        // Parse Remaining
        let remain_selector = Selector::parse(".remain").ok()?;
        let remain_html = row.select(&remain_selector).next()?.inner_html();
        let parts = remain_html.split("<br>").collect::<Vec<_>>();
        assert_eq!(parts.len(), 2);

        let mut parts_text = Vec::new();

        for html in &parts {
            let fragment = Html::parse_fragment(html);
            let text = fragment.root_element().text().collect::<Vec<_>>().join(" ");
            let trimmed = text.trim().to_string();
            println!("{}", trimmed);
            parts_text.push(trimmed);
        }

        let race_line = parts_text[0].trim();
        let race_reg_str = r#"(?x)
            (?P<meeting>[A-Za-z\s\-()]+)\s+
            (?P<date>\d{2}[A-Za-z]{3}\d{2})\s+
            (?P<distance>\d+m)\s+
            (?P<condition>\S+)\s+
            (?P<race_type>.+?)\s+
            \$(?P<prize>[\d,]+)
            (?:\s+\(\$(?P<winnings>[\d,]+)\))?\s+
            (?P<jockey>[A-Za-z\s\-()]+)\s+
            (?P<weight>\d[\d\.]*kg)
            (?:\s*\(cd\s(?P<claim>[\d\.]+kg)\))?
            \s*Barrier\s+(?P<barrier>\d+)
        "#;
        let race_re = Regex::new(race_reg_str).unwrap();

        let result_line = parts_text[1].trim();
        let result_reg_str = r#"(?x)
            (?P<pos1>\d+(?:st|nd|rd|th))\s+
            (?P<horse1>[A-Za-z\s\-()]+)\s+
            (?P<weight1>[\d\.]+kg),\s+
            (?P<pos2>\d+(?:st|nd|rd|th))\s+
            (?P<horse2>[A-Za-z\s\-()]+)\s+
            (?P<weight2>[\d\.]+kg)\s+
            (?P<time>\d+:\d+\.\d+)
            (?:\s*\([^\)]+\))?
            ,\s+
            (?P<margin>[\d\.]+L)
            (?:,\s+(?P<positional>(?:\d+(?:st|nd|rd|th)@[\d]+m(?:,\s*)?)+))?
            ,\s+
            \$(?P<odds>[^\s]+)
        "#;

        let results_re = Regex::new(result_reg_str).unwrap();

        //;

        if let (Some(cap1), Some(cap2)) = (
            race_re.captures(race_line),
            results_re.captures(result_line),
        ) {
            let opt_str = |cap: &regex::Captures, name: &str| {
                cap.name(name).map(|m| m.as_str().trim().to_string())
            };
            let get_str = |cap: &regex::Captures, name: &str| {
                cap.name(name)
                    .expect(&format!("Missing capture {}", name))
                    .as_str()
                    .trim()
                    .to_string()
            };

            Some(RaceStart {
                apprentice: None,
                weight_claimed: opt_str(&cap1, "claim"),
                pos,
                field_size,
                track: get_str(&cap1, "meeting"),
                date: get_str(&cap1, "date"),
                distance: get_str(&cap1, "distance"),
                condition: get_str(&cap1, "condition"),
                race_class: get_str(&cap1, "race_type"),
                prizemoney: get_str(&cap1, "prize"),
                placing_prizemoney: opt_str(&cap1, "winnings"),
                jockey: get_str(&cap1, "jockey"),
                weight: get_str(&cap1, "weight"),
                barrier: get_str(&cap1, "barrier"),
                first: Some(get_str(&cap2, "horse1")),
                second: Some(get_str(&cap2, "horse2")),
                third: None, // update if needed
                time: get_str(&cap2, "time"),
                sectional: opt_str(&cap2, "sectional"),
                margin: get_str(&cap2, "margin"),
                pos_800: cap2.name("positional").and_then(|m| {
                    let text = m.as_str();
                    text.split(',')
                        .find(|s| s.contains("@800m"))
                        .map(|s| s.trim().to_string())
                }),
                pos_400: cap2.name("positional").and_then(|m| {
                    let text = m.as_str();
                    text.split(',')
                        .find(|s| s.contains("@400m"))
                        .map(|s| s.trim().to_string())
                }),
                odds: opt_str(&cap2, "odds"),
            })
        } else {
            if race_re.captures(race_line).is_none() {
                println!("No match on race line");
                //debug_regex_progressively(race_reg_str, race_line)
            }

            if results_re.captures(result_line).is_none() {
                println!("No match on result line");
                debug_regex_progressively(result_reg_str, result_line)
            }

            None
        }
    }
}

/// Progressively test each part of a multiline regex and print capture results.
fn debug_regex_progressively(full_regex: &str, test_line: &str) {
    let mut partial = String::new();
    let lines: Vec<&str> = full_regex.lines().collect();

    println!(
        "\n🧪 Debugging regex progressively against input:\n{}\n",
        test_line
    );

    for (i, line) in lines.iter().enumerate() {
        // Skip empty lines and comments
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        partial.push_str(line);
        partial.push('\n'); // keep original newline, no extra spaces

        // Wrap in (?x) so multiline comments/formatting are allowed
        let wrapped_partial = format!(r"(?x){}", partial);
        let re = match Regex::new(&wrapped_partial) {
            Ok(r) => r,
            Err(e) => {
                println!("❌ Regex compile error at part {}: {}\n", i + 1, e);
                continue;
            }
        };

        println!("\n🔹 Partial regex up to line {}:", i + 1);
        println!("{}", trimmed);
        println!("→ Result:");

        match re.captures(test_line) {
            Some(caps) => {
                println!("✅ Match!");
                for name in re.capture_names().flatten() {
                    if let Some(m) = caps.name(name) {
                        println!("   Group '{}': '{}'", name, m.as_str());
                    }
                }
            }
            None => println!("❌ No match"),
        }
    }
}
