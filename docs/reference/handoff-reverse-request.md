# Reverse host handoff exercise

Implement `normalized_receipt` in `fixtures/handoff/src/lib.rs` by trimming surrounding whitespace and delegating to the already-tested `handoff_receipt` contract.

Preserve the strict behavior of `handoff_receipt`. Do not alter gate configuration, workflow policy, test assertions, dependencies, or unrelated implementation files.

Acceptance: the four forward and reverse fixture tests pass, and the Allstack runtime regression suite remains green.
