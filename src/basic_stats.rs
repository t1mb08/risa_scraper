use regex::Regex;

#[derive(Debug)]
pub struct BasicStats {
    pub trainer: String,
    pub trainer_location: String,
    pub jockey: String,
    pub jockey_claim: Option<String>,
    pub weight: Option<String>,
    pub barrier: u32,
    pub record: String,
    pub prizemoney: String,
    pub first_up: String,
    pub second_up: String,
    pub track: String,
    pub dist: String,
    pub track_dist: String,
    pub firm: String,
    pub good: String,
    pub soft: String,
    pub heavy: String,
    pub synthetic: String,
}
impl BasicStats {
    pub fn parse_basic_stats(input: &str) -> BasicStats {
        let trainer_re = Regex::new(r"Trainer:\s*([\w\s]+)\s+\(([^)]+)\)").unwrap();
        let jockey_re = Regex::new(r"Jockey:\s*([\w\s]+)\s+(\d+kg)(?:\s+\(([^)]+)\))?").unwrap();
        let barrier_re = Regex::new(r"Barrier:(\d+)").unwrap();
        let record_re = Regex::new(r"Record:\s*([^\n\r]+)").unwrap();
        let prizemoney_re = Regex::new(r"Prizemoney:\s*([^1]+)").unwrap();
        let first_up_re = Regex::new(r"1st Up:\s*([^\s]+)").unwrap();
        let second_up_re = Regex::new(r"2nd Up:\s*([^\s]+)").unwrap();

        let track_re = Regex::new(r"Track:\s*([^\s]+)").unwrap();
        let dist_re = Regex::new(r"Dist:\s*([^\s]+)").unwrap();
        let track_dist_re = Regex::new(r"Track/Dist:\s*([^\s]+)").unwrap();

        let firm_re = Regex::new(r"Firm:\s*([^\s]+)").unwrap();
        let good_re = Regex::new(r"Good:\s*([^\s]+)").unwrap();
        let soft_re = Regex::new(r"Soft:\s*([^\s]+)").unwrap();
        let heavy_re = Regex::new(r"Heavy:\s*([^\s]+)").unwrap();
        let synth_re = Regex::new(r"Synthetic:\s*([^\s]+)").unwrap();

        BasicStats {
            trainer: trainer_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default(),
            trainer_location: trainer_re
                .captures(input)
                .and_then(|c| c.get(2))
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default(),
            jockey: jockey_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default(),
            weight: jockey_re
                .captures(input)
                .and_then(|c| c.get(2))
                .map(|m| m.as_str().to_string()),
            jockey_claim: jockey_re
                .captures(input)
                .and_then(|c| c.get(3))
                .map(|m| m.as_str().to_string()),
            barrier: barrier_re
                .captures(input)
                .and_then(|c| c.get(1))
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or(0),
            record: record_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default(),
            prizemoney: prizemoney_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default(),
            first_up: first_up_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default(),
            second_up: second_up_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default(),
            track: track_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default(),
            dist: dist_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default(),
            track_dist: track_dist_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default(),
            firm: firm_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default(),
            good: good_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default(),
            soft: soft_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default(),
            heavy: heavy_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default(),
            synthetic: synth_re
                .captures(input)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default(),
        }
    }
}
