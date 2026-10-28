# Native runtime evidence and Prime control-value qualification

Status: accepted. The user authorized `feat/native-runtime-evidence` in Allstack and `fix/prime-toon-control-values` in AllSource, one PR each.

## Complete job

A developer updates Allstack, invokes native skills in a real AllSource checkout, carries accepted source and ownership between authenticated Codex and Claude, verifies the final application artifact, and publishes one exact-tested commit per repository. AllSource retains delivery history, Chronis owns task claims/completion, and Prime retains source-linked context.

The native skills remain the main interface. This change extends their existing instructions without introducing a workflow catalog entry, another command family, or a mandatory managed run for independent planning/review/QA.

## Observed evidence

- AllSource PR #369 merged as `fc908dd3227c64049820a95ef1ad66432610e6f7`. Its tree equals tested feature commit `0adf7d265eba0650775a012cb519f89b08447c93`: `0f8ecdc1ec89d9495912da8df9e9d494d732c343`.
- The indexed [Prime verification report](https://github.com/all-source-os/all-source/blob/fc908dd3227c64049820a95ef1ad66432610e6f7/docs/reports/2026-10-27-prime-toon-native-verification.md) records real native host calls, retained-memory checks and installed-runtime drift.
- The shared Allstack executable changed during that qualification. The current update restores the artifact from successful CI 40518653382, SHA-256 `c29cc5f1d143878e38e726e4a1fc45d6cfb40b0b5e49b674067407fb77724d78`. Artifact tree `2739520c7835934f00ed344465c192f607eb5a28` equals merged Allstack main `12af0eeb2004c91d6d3d5b3000c4a7de90d0dd1c`. The previous executable is backed up and this run has a pinned copy.
- Copied Rust integration executables can retain a compile-time `CARGO_BIN_EXE_*` path. Keeping their bytes alone does not preserve a later test's runtime dependency.
- After cleanup, Allstack recall requires a Git repository even when archived Prime data exists. The prior archive was successfully reopened through Prime's own MCP engine: five knowledge nodes and two delivery nodes, with the final QA source digest verified.
- A new real MCP probe preserves five JSON properties but produces a TOON response rejected by the pinned 4.4.0 decoder: `Line 5: Invalid escape sequence: \f`. The value is document text containing a comma and form-feed. This is a presentation-encoder defect.

The earlier proposed Prime decision has a historical source digest; its accepted successor and qualification report match current source. Recalled proposals remain context, not permission.

The next application has a 375-case independent reference corpus frozen before implementation, SHA-256 `94117f98fd9a753e2cd977cb236d777cc54dc31fe03d0c3b47cf04093f38a165`. Real authenticated Claude planning read its installed method and producer/tests/policy through sixteen observed calls, fourteen successful. Candidate implementation and execution remain pending.

## Method updates

| Method | Narrow instruction change |
| --- | --- |
| `allstack-handoff` | Bind actual toolkit/host executable paths, digests and bundled methods when an installed runtime changes or multiple installations exist. Carry the chosen runtime into the receiving host rather than trusting PATH or a version string. |
| `allstack-qa` | For serialization changes, freeze representative input and independent expected output before implementation. Check persisted JSON and decoded output, then qualify final immutable runtime artifacts. Distinguish copied harness bytes from embedded executable paths. |
| `allstack-ship` | Preserve required runtime dependencies as well as streams before cleanup. Inspect actual forge enforcement separately from merge mode. Keep final artifact/source identity and exact-commit gate claims distinct. |
| `allstack-recall` | Distinguish Git-bound coordinator commands from Prime store recovery. Reopen an archived copy through its own engine, probe identity/counts, and validate source digests without inventing replacement memory. |

Keep these conditional instructions concise and local to the relevant methods. Update the native reference and generated guide only where their current contract needs clarification. Do not add tests that merely match prose.

## Real application qualification

Use the indexed AllSource control-value plan. Authenticated Claude plans against current source; authenticated Codex implements Rust encoder/test changes; independent Claude reviews and exercises actual MCP processes and the pinned decoder. Native ownership moves Codex → Claude → Codex, with actual resume calls and Chronis claimant readbacks.

The application repair preserves stored JSON and ordinary compact strings, including existing entity-ID behavior. C0-containing values use TOON-specific quoting/escaping. Existing key coverage remains active. Reference JavaScript is used only for independent decoding/corpus generation because that implementation is published as a JavaScript package; application code and permanent tests remain Rust.

## Verification and publication

1. Validate changed skills structurally with the skill-creator validator and inspect both generated host variants. Verify conflicts remain preserved.
2. Build/install an exclusively owned Allstack candidate from the intended source under the existing compiler/deadline rules. Pin it for both hosts and verify its actual packaged methods; version equality alone is insufficient.
3. Execute existing Allstack policy and changed-component checks. Use the updated installed methods in the real AllSource qualification, retaining actual calls and outputs.
4. Freeze regression inputs before application implementation. Establish a negative merged-source baseline, then prove the candidate on the same corpus and retained store.
5. Run each repository's unchanged exact-commit policy, publish one commit and one PR per repository, read remote heads, and inspect CI. Do not merge or deploy.
6. Read AllSource history, Chronis completion and matching Prime context through their engines. Archive receipts, raw streams, necessary binaries, source documents and project state before removing owned worktrees/build directories.

Confidence: rock. The failure, output owner and compatibility boundary are observed. Compiler contention, source/runtime drift and missing external permissions remain explicit prerequisites rather than inferred successes.

## Requested branches

- Allstack: `feat/native-runtime-evidence`, one PR for method updates, indexed evidence and required fixes.
- AllSource: `fix/prime-toon-control-values`, one PR for the application repair and real-host evidence.

Allstack PR #8 belongs to separate existing work and is outside this proposal.
