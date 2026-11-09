//! Pure rules for design review and approval. I/O lives in the infrastructure layer.

/// The reviewer receives the draft path and repository root only, never the author's conversation.
pub fn review_prompt(path: &str, root: &str) -> String {
    format!(
        "You are an independent design reviewer with no prior conversation. Review the design document at `{path}` in the repository at `{root}`. Read that document and the repository code it names; do not read anything it does not point to. The review has a fixed deadline and output appears only when you finish: check at most 12 code locations, starting with the claims the design depends on most, then write the report and name any claim you left unchecked. Do not edit files or run commands that change state. Report each defect as: severity, location, consequence, and evidence. End with exactly one line: `VERDICT: clean` when no defect remains, otherwise `VERDICT: findings`."
    )
}

/// Returns `unknown` unless the reviewer wrote an explicit verdict line.
pub fn verdict(text: &str) -> &'static str {
    let mut verdict = "unknown";
    for line in text.lines().map(str::trim) {
        match line {
            "VERDICT: clean" => verdict = "clean",
            "VERDICT: findings" => verdict = "findings",
            _ => {}
        }
    }
    verdict
}

pub fn approver_valid(name: &str) -> bool {
    let name = name.trim();
    !name.is_empty() && name.len() <= 128 && !name.chars().any(char::is_control)
}
