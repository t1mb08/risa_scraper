use regex::Regex;

#[allow(dead_code)]
pub fn debug_regex_progressively(full_regex: &str, test_line: &str) {
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
