use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Ok(());
    }
    let path = &args[1];
    let content = fs::read_to_string(path)?;

    let mut body_lines = Vec::new();
    let mut human_trailers = Vec::new();
    let mut ai_tools = Vec::new();

    let ai_re = Regex::new(r"(?i)^Co-authored-by: (?P<name>[^<]+)<noreply@[^>]*anthropic\.com>")?;
    let any_coauthor_re = Regex::new(r"(?i)^Co-authored-by: .*")?;

    for line in content.lines() {
        if let Some(caps) = ai_re.captures(line) {
            ai_tools.push(format!("Used-tool: {}", caps["name"].trim()));
        } else if any_coauthor_re.is_match(line) {
            human_trailers.push(line.to_string());
        } else {
            body_lines.push(line.to_string());
        }
    }
    let mut final_msg = body_lines.join("\n").trim().to_string();
    if !ai_tools.is_empty() {
        final_msg.push_str("\n\n");
        final_msg.push_str(&ai_tools.join("\n"));
    }
    if !human_trailers.is_empty() {
        final_msg.push_str("\n\n");
        final_msg.push_str(&human_trailers.join("\n"));
    }
    fs::write(path, final_msg)?;
    Ok(())
}
