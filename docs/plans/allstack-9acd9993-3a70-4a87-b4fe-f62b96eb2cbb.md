1. **Outcome and complete flow**

   Caller supplies `from` and `to` host strings. `handoff_receipt` returns `Ok("codex -> claude-code")` or `Ok("claude-code -> codex")` for supported transfers. Every other input pair returns `Err`, including empty strings, unknown hosts, unchanged hosts, case variants, and whitespace-padded names.

   Managed flow: coordinator indexes this plan; incoming build host reads indexed plan and request, changes fixture function, and returns candidate for coordinator checkpointing. Coordinator runs frozen verification against resulting candidate. Task ownership, host transitions, commits, and publication remain coordinator-controlled.

2. **Source context and precise change**

   Requirements confirmed in `docs/reference/handoff-forward-request.md`; workflow context recovered from indexed `docs/plans/2026-10-24-cross-host-workflow-design.md`.

   Change only `fixtures/handoff/src/lib.rs:1`, where `handoff_receipt` currently returns `Err("not implemented")`:

   - Preserve public signature: `(&str, &str) -> Result<String, &'static str>`.
   - Rename `_from` and `_to` parameters to `from` and `to`.
   - Match exactly `("codex", "claude-code")` and `("claude-code", "codex")`; return `Ok(format!("{from} -> {to}"))`.
   - Return static error such as `"invalid host transfer"` for remaining pairs. Exact error wording has no specified contract.
   - Perform no trimming, whitespace normalization, or case conversion.

   Preserve `normalized_receipt` at line 5 byte-for-byte, including its current placeholder. Preserve tests, manifests, lockfiles, gates, workflow policy, and unrelated implementation files. No dependencies or new files required.

3. **Acceptance checks**

   Coordinator executes existing `workflows/forward-gates.toml` unchanged, sequentially under its configured 240-second command limit:

   | Check | Command | Required evidence |
   |---|---|---|
   | Runtime regression | `cargo nextest run --locked --offline` | Successful exit, positive executed-test count, `tests run:` marker |
   | Forward fixture | `cargo test --manifest-path fixtures/handoff/Cargo.toml --offline --test forward` | Both existing tests execute and pass; `2 passed` marker |
   | Candidate whitespace | `git diff --check HEAD^ HEAD` | Successful exit on coordinator-prepared commit |

   Review final diff to confirm only `handoff_receipt` changed. Inspect exhaustive match to verify rejection of whitespace-padded inputs, empty destinations, and `claude-code` self-transfers; existing forward assertions do not cover every required invalid pair. Do not alter assertions.

   Use targeted `--test forward`; reverse tests exercise separate, intentionally preserved placeholder. Root runtime suite and fixture occupy separate Cargo workspaces.

4. **Unresolved prerequisites and limits**

   No functional ambiguity remains. Later verification requires Rust 1.95 or newer, Cargo, cargo-nextest, offline dependency availability, writable build output, and uncontended compile capacity. Those prerequisites remain unverified.

   Coordinator must supply current authoritative epoch and active worktree after any handoff, retain verification receipts, and reconcile existing commit/publication authority before those stages. Current supplied context: run `9acd9993-3a70-4a87-b4fe-f62b96eb2cbb`, epoch 1, revision 5, task `t-3d5fb1`; detached checkout intentional.

   Planning inspection only. No source changes, compilation, tests, or state transitions performed.
