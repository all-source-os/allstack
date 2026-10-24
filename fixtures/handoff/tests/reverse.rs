use handoff_example::normalized_receipt;

#[test]
fn whitespace_is_normalized_before_validating_hosts() {
    assert_eq!(normalized_receipt(" claude-code\n", "\tcodex ").unwrap(), "claude-code -> codex");
    assert_eq!(normalized_receipt(" codex ", " claude-code ").unwrap(), "codex -> claude-code");
}

#[test]
fn normalization_cannot_hide_unknown_or_same_host_transfers() {
    assert!(normalized_receipt(" codex ", "\tcodex").is_err());
    assert!(normalized_receipt("unknown ", "codex").is_err());
    assert!(normalized_receipt(" ", "claude-code").is_err());
}
