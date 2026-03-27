use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(p) => p,
        None => return Ok(()),
    };
    let content = fs::read_to_string(path)?;

    let mut body_lines = Vec::new();
    let mut human_trailers = Vec::new();
    let mut ai_tools = Vec::new();

    let ai_re = Regex::new(r"(?i)^Co-authored-by: (?P<name>[^<]+)<noreply@[^>]*anthropic\.com>")?;
    let any_coauthor_re = Regex::new(r"(?i)^Co-authored-by: .*")?;

    for line in content.lines() {
        if let Some(caps) = ai_re.captures(line) {
            let tool_name = caps["name"].trim();
            ai_tools.push(format!("Used-tool: {}", tool_name));
        } else if any_coauthor_re.is_match(line) {
            human_trailers.push(line.to_string());
        } else {
            body_lines.push(line);
        }
    }
    let mut sections = vec![body_lines.join("\n").trim().to_string()];
    if !ai_tools.is_empty() {
        sections.push(ai_tools.join("\n"));
    }
    if !human_trailers.is_empty() {
        sections.push(human_trailers.join("\n"));
    }
    let final_msg = sections
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n");
    fs::write(path, final_msg)?;
    Ok(())
}
