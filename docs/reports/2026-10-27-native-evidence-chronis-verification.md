# Native evidence checkpoints and real Chronis delivery

## Delivered behavior

The seven native skills remain the user interface. Updated planning and review methods require producer/history compatibility and an existing consumer's next operation. QA retains failed responses and distinguishes actual host calls, running artifacts and final-commit gates. Handoff requires the receiving host to read its installed method and current source. Shipping archives epoch-local streams before cleanup and reads tracker/memory state through their engines.

The [accepted design](../plans/2026-10-27-native-evidence-and-chronis-design.md) and [request](../reference/native-evidence-request.md) define this change. Existing AllSource history, Chronis ownership and project Prime remain underneath; no additional orchestrator or workflow catalog entry was introduced.

## Real application delivered

[AllSource PR #368](https://github.com/all-source-os/all-source/pull/368) publishes exact verified commit `d6f1b0673785cd53e1c1faa5e90ad677f66ecdd0`. Its [versioned application report](https://github.com/all-source-os/all-source/blob/d6f1b0673785cd53e1c1faa5e90ad677f66ecdd0/docs/reports/2026-10-27-chronis-native-verification.md) records source, executable and receipt identities.

Chronis now escapes backslashes, pipes, line controls and non-final action-field colons at the presentation boundary. Stored titles and claimant identities remain unchanged. Delimiter-bearing identities cannot insert an extra column or status field. Ordinary output, legacy IDs and silent repeated completion retain their contracts.

| Evidence | Observed result |
| --- | --- |
| Frozen four-case acceptance against freshly built baseline | 1 passed, 3 failed; expected exit 101 |
| Same acceptance against rebuilt candidate | 4 passed, 0 failed, 0 ignored |
| Independent authenticated Claude execution | Same 4 cases passed through installed Allstack |
| Final exact-commit Chronis policy | Formatting, 102 executed tests, strict all-target Clippy and whitespace passed |
| Native destination readback | Remote branch and PR head equal the verified commit; one commit |
| Actual Chronis readback | Task `t-691ec2` done; reason: remote commit and PR head verified |
| Actual project Prime readback | Source-linked plan/review/QA records and published outcome retrieved through Allstack |

Acceptance SHA-256 is `46eeaa13966fc153bbc65fb10f36ba9c082474e4dfb0ba0f77cb4e9e77df534a` in both builds. The test executes separate real CLI processes for create, claim, list/show, completion, rejected transitions and JSON state readback. No direct storage-file parsing substitutes for Chronis.

The baseline CLI independently reproduced title collision and an injected status-looking line while the actual task remained in progress. Baseline and candidate use the same one-line corrected standalone lockfile. Two inherited Clippy warnings and existing claim-test formatting were repaired without suppressions or reduced checks.

## Actual hosts and ownership

Authenticated Claude session `49403bff-e4f2-4447-94ef-9045edaccfd1` read the installed planning method and current source: 12 observed file calls. Real Codex session `01a17d58-bec1-7a03-a656-ef5d5a3e9836` wrote the application formatter and acceptance test. An initial bounded inspection timed out after edits; retained source and receipts supported continuation without replaying implementation.

Independent Claude source review identified colon ambiguity for legacy action IDs. Codex corrected that boundary and the completion expectation after actual dispatch inspection. Claude QA session `f9da996b-5cb5-4dbc-a154-4fb612330dfc` read its installed QA method, checked executable hashes and executed the acceptance binary through Allstack. Inner receipt `34f692ef-8dd8-4dc6-b278-0a6392df64f6` records four passing tests. Parent readback confirmed the original CLI hash after QA.

Native run `b67f2a72-0d88-40b5-bf4d-efdc492f4f99` moved Codex epoch 1 → Claude epoch 2 → Codex epoch 3. Both receiving hosts read installed handoff instructions and successfully resumed. Actual Chronis claimant readbacks matched each host and epoch. Codex's initial read-only resume failed permission checks despite a successful host-process exit; the subsequent workspace-permitted receipt establishes successful continuation. Final independent Claude inspection also verified the behavior-preserving Kanban match guards.

## Installed artifact and package integrity

The workspace-local installed Allstack executable has SHA-256 `c29cc5f1d143878e38e726e4a1fc45d6cfb40b0b5e49b674067407fb77724d78`. It is the macOS artifact from [green CI run 40510117757](https://github.com/all-source-os/allstack/actions/runs/40510117757). CI source was synthetic merge `7efd61db76e2cb980145e7074f637d7cf10affaf`; its Git tree `9742a7b59db3dcdb4a9886c265ba3c428bb9740c` equals feature head `0f7f764b492ac9ff9c5b4906201adc516c79b7aa`. Later changes add verification documentation, policy environment binding and a byte-preserving Git whitespace attribute; bundled methods and binary inputs remain the same.

Packaging qualification compared all 14 generated Codex/Claude skill files, created no stores and preserved a deliberately conflicting customization. The installed fixed-dependency executable independently lists the same seven skills without opening stores. Application installation replaced only resources verified against their previous generated contents. Explicit host directories in PATH resolved Claude discovery; actual application doctor then reported setup and executables ready.

Allstack's complete native policy executed 74 toolkit tests with two configured skips, eight expense-example tests and four handoff-fixture tests. Toolkit/example Clippy, first-party formatting and source whitespace checks passed. The byte-verified upstream help fixture has a file-specific trailing-space attribute; other whitespace checks remain active. Actual authenticated Chronis work above supplies real host evidence independently of fixture execution.

Strict dependency auditing exposed inherited compression advisories. The [indexed compression decision](../decisions/2026-10-27-parquet-compression-patch.md) records a manifest-only patch to the exact published Parquet archive. All 227 upstream files are checked by an integrity test; only two zstd requirements change. Fixed versions are zstd 0.14.3, zstd-safe 8.2.0 and zstd-sys 2.1.2. Unique package count remains 258. Local cargo-deny and final-current Linux/macOS CI passed; no advisory exception was added.

## Evidence retention and limits

Before application shipping removed accepted epoch worktrees, 258 evidence files were copied to ignored proof storage. All 19 command receipts and 57 raw streams were checked against recorded lengths and SHA-256 digests. Exported native history and final gate receipts preserve AllSource engine readback. Published-outcome Prime node `bc3cf8f2-3f3f-4876-a271-979a3e698cd4` points to PR #368. Private transcripts and source-linked local state remain outside tracked deliverables.

Qualification covers sequential real authenticated hosts on this machine and isolated local task data. It does not establish a second physical machine, a new hosted sync revision, browser acceptance, merge or deployment. Dependency-ID comma subformat remains unchanged. A failed nested-epoch Cargo preflight and initial host timeouts remain distinguishable from executed tests.

Neither repository currently has applicable branch protection/rulesets; the configured hook directory has no commit/push hooks. CI is visible but does not enforce merge. Publication uses explicit native exact-commit verification and remote destination readback.
