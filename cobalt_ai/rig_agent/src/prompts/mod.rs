use std::fs;
use std::collections::HashMap;

pub fn load_templates() -> HashMap<String, String> {
    let mut templates = HashMap::new();
    let file_content = fs::read_to_string("src/prompts/templates.frs")
        .or_else(|_| fs::read_to_string("rig_agent/src/prompts/templates.frs"))
        .unwrap_or_default();

    let mut current_key = String::new();
    for line in file_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_key = trimmed[1..trimmed.len() - 1].to_string();
        } else if trimmed.starts_with("system_prompt =") {
            if !current_key.is_empty() {
                let prompt_val = trimmed["system_prompt =".len()..]
                    .trim()
                    .trim_matches('"')
                    .to_string();
                templates.insert(current_key.clone(), prompt_val);
            }
        }
    }
    templates
}
