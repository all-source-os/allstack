# Fresh repository delivery exercise

Implement `handoff_receipt` in the initialized example repository's `src/lib.rs`:

```rust
pub fn handoff_receipt(from: &str, to: &str) -> Result<String, &'static str>
```

Accept exactly `codex` and `claude-code`. A transfer must change hosts. Return `FROM -> TO` for either supported direction. Reject empty, unknown, unchanged, whitespace-padded, and differently cased host names. Do not normalize input.

Keep the public signature and existing test assertions. Change only `src/lib.rs`; preserve the frozen verification policy, workflow methods, documentation, repository instructions, host skills, and dependency configuration. The coordinator owns Chronis, commits, branches, verification, and publication. Do not change those from a managed host call. Use normal host permissions without bypass flags.

Acceptance: all five integration tests execute and pass through the generated `workflows/verification.toml`; formatting and whitespace gates pass on the exact delivered commit. Retained evidence must show authenticated Codex planning, an immutable handoff to Claude Code for implementation, a fresh Codex epoch recovering authoritative context, verified source, and a matching remote commit/open PR. Chronis task completion and source-linked Prime outcome must be read back after publication.

The example begins with a failing valid-transfer assertion. A planning response, compilation, manually edited result, or zero selected tests cannot establish completion.
