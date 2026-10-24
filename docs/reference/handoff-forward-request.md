# Forward host handoff exercise

Implement `handoff_receipt` in `fixtures/handoff/src/lib.rs`.

Accept exactly `codex` and `claude-code` as hosts. A transfer must change hosts. For a valid transfer return `FROM -> TO`; reject unknown, empty, or unchanged hosts. Do not normalize whitespace in this function.

Preserve `normalized_receipt` for the separate reverse-host exercise. Do not alter gate configuration, workflow policy, test assertions, dependencies, or unrelated implementation files.

Acceptance: the two tests in `fixtures/handoff/tests/forward.rs` pass, and the Allstack runtime regression suite remains green.
