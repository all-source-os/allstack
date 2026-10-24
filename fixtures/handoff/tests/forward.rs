use handoff_example::handoff_receipt;

#[test]
fn both_supported_directions_have_canonical_receipts() {
    assert_eq!(handoff_receipt("codex", "claude-code").unwrap(), "codex -> claude-code");
    assert_eq!(handoff_receipt("claude-code", "codex").unwrap(), "claude-code -> codex");
}

#[test]
fn unknown_or_unchanged_hosts_are_rejected() {
    assert!(handoff_receipt("unknown", "codex").is_err());
    assert!(handoff_receipt("codex", "unknown").is_err());
    assert!(handoff_receipt("codex", "codex").is_err());
    assert!(handoff_receipt("", "claude-code").is_err());
}
