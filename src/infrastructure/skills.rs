//! Native host packaging shares methods without imposing managed-run capabilities.
use anyhow::{Result, bail};
use serde_json::{Value, json};

const METHODS: &[(&str, &str)] = &[
    (
        "shape",
        include_str!("../../skills/allstack-shape/SKILL.md"),
    ),
    ("plan", include_str!("../../skills/allstack-plan/SKILL.md")),
    (
        "review",
        include_str!("../../skills/allstack-review/SKILL.md"),
    ),
    ("qa", include_str!("../../skills/allstack-qa/SKILL.md")),
    ("ship", include_str!("../../skills/allstack-ship/SKILL.md")),
    (
        "recall",
        include_str!("../../skills/allstack-recall/SKILL.md"),
    ),
    (
        "handoff",
        include_str!("../../skills/allstack-handoff/SKILL.md"),
    ),
];

pub fn method(name: &str) -> Result<&'static str> {
    let name = name.strip_prefix("allstack-").unwrap_or(name);
    match METHODS.iter().find(|(id, _)| *id == name) {
        Some((_, text)) => Ok(text),
        None => bail!("unknown skill {name}; use allstack skills for available methods"),
    }
}

pub fn inspect(name: Option<&str>) -> Result<Value> {
    Ok(match name {
        Some(name) => json!({"method":method(name)?,"stores_opened":false}),
        None => {
            json!({"skills":METHODS.iter().map(|(id,_)| format!("allstack-{id}")).collect::<Vec<_>>(),
            "hosts":["codex","claude-code"],"stores_opened":false})
        }
    })
}

pub fn resources() -> Vec<(String, String)> {
    let mut files = Vec::new();
    for (host, overlay) in [
        (
            ".agents",
            "Codex: invoke as $allstack-<name> or request its name naturally. Use native repository tools and its current sandbox. A nested host command requires the same applicable authority as a direct call.",
        ),
        (
            ".claude",
            "Claude Code: invoke as /allstack-<name> or request its name naturally. Use native repository tools and its current permission mode. An unavailable or denied tool leaves its check unverified.",
        ),
    ] {
        for (id, method) in METHODS {
            files.push((
                format!("{host}/skills/allstack-{id}/SKILL.md"),
                format!("{method}\n## Host capability\n\n{overlay}\n"),
            ));
        }
    }
    files
}
