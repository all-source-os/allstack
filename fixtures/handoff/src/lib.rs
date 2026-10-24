pub fn handoff_receipt(from: &str, to: &str) -> Result<String, &'static str> {
    match (from, to) {
        ("codex", "claude-code") | ("claude-code", "codex") => Ok(format!("{from} -> {to}")),
        _ => Err("invalid host transfer"),
    }
}

pub fn normalized_receipt(from: &str, to: &str) -> Result<String, &'static str> {
    handoff_receipt(from.trim(), to.trim())
}
