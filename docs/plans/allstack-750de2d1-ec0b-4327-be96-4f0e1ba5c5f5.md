Write tool disabled — this planning host is restricted to `Read`/`Glob`/`Grep`, as the verification report records. Delivering the plan in this response as instructed.

---

# Plan: implement `normalized_receipt` (reverse host handoff exercise)

## Context

Run `750de2d1-ec0b-4327-be96-4f0e1ba5c5f5`, Claude Code epoch 1, Chronis task `t-649659`. This is the reverse leg of the cross-host workflow verification in `docs/reports/2026-10-24-cross-host-workflow-verification.md`. The forward leg delivered the strict `handoff_receipt` contract (commit `112a738`). The reverse leg adds a whitespace-tolerant wrapper, `normalized_receipt`, that normalizes its inputs and then defers every validation decision to the strict, already-tested function.

The wrapper currently exists as a stub returning `Err("not implemented")`; its two reverse fixture tests fail. The goal is a one-function implementation that makes the four fixture tests pass without weakening `handoff_receipt` and without touching anything else.

## Contract under implementation

`fixtures/handoff/src/lib.rs` is a self-contained Cargo package (`handoff-example`, edition 2024, `publish = false`, empty `[workspace]` so it is isolated from the parent Allstack workspace).

Current source:

```rust
pub fn handoff_receipt(from: &str, to: &str) -> Result<String, &'static str> {
    match (from, to) {
        ("codex", "claude-code") | ("claude-code", "codex") => Ok(format!("{from} -> {to}")),
        _ => Err("invalid host transfer"),
    }
}

pub fn normalized_receipt(_from: &str, _to: &str) -> Result<String, &'static str> {
    Err("not implemented")
}
```

`handoff_receipt` is the strict contract: it accepts only the two canonical host pairs and rejects everything else (unknown host, same host, empty string). It must remain exactly as above.

## The change

Replace the `normalized_receipt` stub body (lines 8–10 of `fixtures/handoff/src/lib.rs`) with a trim-and-delegate implementation:

```rust
pub fn normalized_receipt(from: &str, to: &str) -> Result<String, &'static str> {
    handoff_receipt(from.trim(), to.trim())
}
```

Rationale:

- `str::trim()` strips leading and trailing whitespace — spaces, `\t`, `\n` — exactly the normalization the reverse tests feed in (`" claude-code\n"`, `"\tcodex "`).
- Delegation keeps all validation in one place. `normalized_receipt` makes no accept/reject decision of its own, so it cannot widen the set of accepted transfers; the strict behavior is preserved by construction. This is the invariant the request names, guarded by the reverse test `normalization_cannot_hide_unknown_or_same_host_transfers`.
- No lifetime problem: the `Ok` arm returns an owned `String` (from `format!`), the `Err` arm is `&'static str`, so the result borrows nothing from the trimmed arguments. The signature is unchanged from the stub, so no caller or test import changes.
- Rename `_from`/`_to` to `from`/`to` (now used).

## Caller flow (end to end)

A caller holding two host identifiers with incidental surrounding whitespace calls `normalized_receipt(from, to)`:

1. Both arguments trimmed.
2. Trimmed pair handed to `handoff_receipt`.
3. Canonical pair (`codex` ↔ `claude-code`, either direction) → `Ok("<from> -> <to>")` built from trimmed values.
4. Any other pair — unknown host, identical host, now-empty string → `Err("invalid host transfer")`.

All four fixture assertions, worked:

| Call | Trimmed | Result |
| --- | --- | --- |
| `normalized_receipt(" claude-code\n", "\tcodex ")` | `claude-code`, `codex` | `Ok("claude-code -> codex")` |
| `normalized_receipt(" codex ", " claude-code ")` | `codex`, `claude-code` | `Ok("codex -> claude-code")` |
| `normalized_receipt(" codex ", "\tcodex")` | `codex`, `codex` | `Err` (same host) |
| `normalized_receipt("unknown ", "codex")` | `unknown`, `codex` | `Err` (unknown host) |
| `normalized_receipt(" ", "claude-code")` | ``, `claude-code` | `Err` (empty host) |

Forward assertions unaffected — `handoff_receipt` untouched.

## Files

- **Edit:** `fixtures/handoff/src/lib.rs` — replace the `normalized_receipt` body only. Single source change.
- **Do not touch:** `handoff_receipt` itself; `fixtures/handoff/tests/forward.rs`; `fixtures/handoff/tests/reverse.rs`; `fixtures/handoff/Cargo.toml`; `fixtures/handoff/Cargo.lock`; any gate, workflow-policy, or parent-workspace file. No dependency added — `str::trim` is standard library.

## Acceptance checks

1. **Fixture suite green (4 tests).** From `fixtures/handoff/`: `cargo test` — 2 pass in `tests/forward.rs` (`both_supported_directions_have_canonical_receipts`, `unknown_or_unchanged_hosts_are_rejected`), 2 pass in `tests/reverse.rs` (`whitespace_is_normalized_before_validating_hosts`, `normalization_cannot_hide_unknown_or_same_host_transfers`). Zero failures, zero skipped.
2. **Allstack runtime regression suite remains green.** Run the parent repository's runtime regression suite (the forward leg recorded 19 executed / 19 passed / 0 skipped). The fixture package is an isolated workspace not imported by the runtime crate, so this is a no-regression safety check, not a surface this change touches.
3. **Whitespace gate clean.** `git diff --check HEAD -- fixtures/handoff/src/lib.rs` exits 0 — the edit introduces no trailing whitespace. (The report notes trailing whitespace in generated output has stopped commit preparation on this run before.)

## Unresolved prerequisites

- **None blocking the edit.** Fully specified by the four test assertions; no ambiguity to resolve with the user.
- **Build-slot contention is an environmental hazard, not a prerequisite.** The report records that competing external Cargo builds previously blocked compilation and that verification correctly waited for the compiler slot to idle rather than terminating a peer. Confirm the compiler slot is free before invoking the fixture `cargo test`; do not kill competing builds to reclaim it.
- **Edition 2024 toolchain.** `Cargo.toml` is `edition = "2024"`; the report cites Rust 1.95 verification toolchain, which satisfies it.

## Notes for the build host

- This plan host made no edits (restricted to `Read`/`Glob`/`Grep`). The build host adds editing and permitted Git commands.
- Entire deliverable is the three-line function body above. Do not expand scope: no refactor of `handoff_receipt`, no new helper, no new test, no manifest change.
- Preserve the exact error string `"invalid host transfer"` by not touching `handoff_receipt`; the reverse error-path tests assert only `is_err()`, but the forward contract treats the strict function as frozen.
