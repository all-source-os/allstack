# Workflow catalog and authenticated bug-fix verification

Date: 2026-10-25. [Implementation PR #4](https://github.com/all-source-os/allstack/pull/4), [catalog design](../plans/2026-10-24-workflow-catalog-design.md), and [repair request](../reference/catalog-bugfix-request.md).

## Observed behavior

The installed CLI lists `delivery` and `bugfix`, selects diagnosis and repair methods for `bugfix`, and retains that identity through AllSource events, Chronis claims, and source-linked Prime records. Actual Codex diagnosed a Prime retry conflict, actual Claude wrote its repair and compatibility tests, and an immutable handoff returned the candidate to Codex epoch 3.

The repair binds each Prime run/phase key to one workflow, run identity, source, and summary. Identical retries remain idempotent. Conflicting retries and duplicate keys fail without mutation. Historical records missing workflow metadata reconcile as delivery only; explicit malformed metadata receives no fallback.

| Retained identity | Value |
| --- | --- |
| Run | `f2c4e41f-7616-4b34-aa56-4347fd1617fc` |
| Selected workflow | `bugfix`, `workflows/bugfix.toml` |
| Workflow SHA-256 | `497e182314e07e811c8892d197aafb47f85ac2a4a91307e7cc6198896a217406` |
| Diagnosis method SHA-256 | `f33a8a24d74fd2c2a11c2e5e7b4f80937d23e59a23f8be7755249250c8631176` |
| Repair method SHA-256 | `0ca4e0b91f542774f9d2f9f192f593f6eb038f0f6d71257551dc7e0f9885c5d2` |
| Accepted Codex plan SHA-256 | `9d878a3844e0473443d1163882c342d77e10053fc6ebef4d9b19d40f4f8d2587` |
| Frozen policy SHA-256 | `4f1c98824edcc5708a7f61e1417c2485e15017ee35e5911dae376607eaa4c44b` |
| Frozen reproduction file SHA-256 | `acd0d8725f9d82dab2d3863053637252c0385fb3f9bbfc21c36249b7b4226bd2` |
| Final installed executable SHA-256 | `576a673ebc9975e09e50f06445131d1ebff2ef7ce0b3df60aa751217bc19d969` |
| Chronis run task | `t-f85189` |
| Publication destination | Existing implementation PR #4; one owned feature-branch commit |

Raw process receipts, account metadata, source snapshots, and event-store files remain private. Retained private manifests bind executable digests to their compiled source inputs. The final installed executable was built from prepared code commit `bb06ce5c828feab6ad012272ff91db7f8e0c4d58`; this identifies the code input before inclusion of this report.

## Baseline and regression evidence

[Baseline CI run 40143180893](https://github.com/all-source-os/allstack/actions/runs/40143180893) tested feature head `fe7423c8f7deb0f6f93228cc764fbd8756fd66eb` through GitHub's synthetic merge checkout. Linux and macOS each executed 39 tests: 38 passed, one failed, one skipped. The conflicting-retry assertion expected an error but received success when workflow changed from `bugfix` to `delivery` under the same key. Formatting, dependency policy, and Clippy passed in that baseline run.

The frozen two-test reproduction remained byte-identical after repair. A focused execution against actual embedded Prime passed both reproduction tests and seven new compatibility tests: nine executed, nine passed. The conflict loop reached workflow, source, and summary variations after repair. Compatibility cases seed historical, duplicate, and malformed records through Prime's own API and reopen the engine before inspecting them.

An expanded focused run executed 18 catalog, architecture, and Prime tests successfully, including selected-definition/method mutation rejection and preservation of frozen paths when catalog entries change. A separate `cargo +1.95 clippy --all-targets --locked --offline -- -D warnings` completed successfully. Missing, mismatched, and duplicate readback validation also has deterministic helper coverage; that coverage does not simulate a real storage outage.

## Installed resources and historical data

An isolated executable prefix, rather than the source-tree binary path, supplied the CLI. A macOS sandbox denied reads of both Allstack source checkouts. Fresh initialization created 16 resources, listed both workflows, initialized actual Chronis, and returned `setup_ready: true` with `chronis: readable`.

Additive initialization was tested over the previously bundled delivery files. It added catalog resources and the documentation link while preserving delivery definitions, methods, guide, both delivery skills, policy bytes, and existing index content. The rebuilt adapter artifact and final installed artifact passed source-denied bootstrap checks as well. Final installed doctor read the actual Chronis store and both catalog entries while source reads remained denied.

The catalog binary also reopened the previous authenticated delivery run `9f981bf4-cb9d-477e-8c5d-8864e1296834` through AllSource's engine. All 22 events validated, revision 22 remained `shipped`, and selection metadata remained absent throughout the legacy serialization. The golden legacy-event test independently checks byte-for-byte serialization compatibility.

## Real hosts, recovery, and permission handling

Codex CLI reported its normal ChatGPT login; Claude reported its normal first-party login. No authentication or permission bypass flags were used.

| Completed host call | Actual receipt |
| --- | --- |
| Codex diagnosis, epoch 1 | Exit 0, `turn.completed`, 29 command completions, 178,421 ms |
| Claude completion, epoch 2 | Exit 0, `subtype: success`, `is_error: false`, zero permission denials, 93,041 ms |
| Claude model label | `claude-opus-4-8` |

The Codex receipt SHA-256 is `d37f468b09851b883062058edb6cff6ef8ba88d5a2bd34a03387be543c1d4a0a`. The successful Claude receipt SHA-256 is `c397f3abfa68583006c74b7a49bd49b6f6e511bd5c53af867af31b925a54509c`.

Two earlier Claude calls reached the frozen 240-second deadline after retaining source changes: 240,021 ms and 240,014 ms, both with `timed_out: true` and `owner_lost: false`. The coordinator left each operation unresolved. Explicit inspection and recovery retained the repair and then the complete compatibility test file without replaying either call.

A later Claude call returned a successful model result but included five denied shell attempts. Its actual receipts showed RTK-prefixed Git commands and file hashing outside the adapter's original allowlist. The coordinator refused to advance. The adapter now permits the corresponding narrow RTK forms and `shasum -a 256`, and explains current-directory shell scope to the host. `git -C`, compound commands, compilation, and publication remain outside that managed shell scope. A subsequent real call completed with zero denials; actual successful calls included file hashing, Git reads, and explicit staging.

Each handoff restored a separate worktree and read back the Chronis claimant. Epoch 3 resumed the retained request, accepted plan, selected methods, and Prime context. The accepted Claude checkpoint was `b08898bddbc09b87f1fce918e72111f25501d30f29e6c32c9341932aa548f44a`.

## Final verification and publication records

The frozen [catalog policy](../../workflows/catalog-gates.toml) requires formatting, the complete positive-execution toolkit suite, Clippy, both host fixtures, and source whitespace checks on the prepared commit. Local compiler contention is checked before compilation. One verification attempt prepared the owned commit but refused to compete when another Cargo process started; focused checks and lint were then run before the final policy execution.

Complete policy execution passed all five gates. The toolkit suite executed 47 tests successfully, with one explicitly ignored real-Chronis qualification. Both host fixtures executed two tests each, all passing; their empty library unit did not supply execution proof. Formatting, Clippy, and whitespace checks returned exit 0. The runtime-suite receipt digest was `33af847303a64ab362905c3bf95c5b473cee9a158df5cdc81fccd7eabeb61b49`. Report inclusion requires a fresh bound verification of the resulting commit before publication.

Authoritative final commit, gate receipts, remote/PR agreement, Chronis completion, and Prime outcome are exposed by the run's `evidence`, `history`, and `recall` commands. The implementation PR links the delivered SHA and final checks. A fresh clone contains code and indexed documents; original private stores and validated exports remain local.

The supported boundary remains sequential hosts on one Unix machine, local embedded storage and lexical Prime recall, and a verified feature commit with an open PR. Merge, deployment, hosted sync, arbitrary workflow graphs, cross-machine transport, and a new review-only lifecycle were not exercised.
