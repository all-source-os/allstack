# Native runtime checks and AllSource Prime validation

## Outcome

A developer invokes native Allstack skills in Codex and Claude, verifies an actual application change with inspectable command receipts, and publishes the exact verified commit through one PR per repository. The seven specialist skills remain the main interface. AllSource retains delivery history, Chronis owns claims and completion, and Prime retains source-linked decisions.

## Evidence from the previous application run

The merged `wolven-tech/autoresearch-rs` PR #4 established the native flow against a real application. Its reviewed commit was `e7338d04fe9f4835bb852c67ad678ad53b9d3dfe`; merged main contains the same source tree. The retained local qualification is `.allstack/proof/autoresearch-native-e7338d0` in the founder-mode workspace.

- Cloning a warm Cargo target directory can preserve artifact timestamps that allow a stale application executable to be reused. A successful build invocation alone does not establish source provenance.
- Native host command output can combine stdout and stderr. Expected application failures can appear as host tool errors, even when their status is the required acceptance result.
- Runtime capture required a compiled example helper and custom orchestration. The process supervisor already owns deadlines, separate streams, and child cleanup; native users need a supported entry point to that capability.
- Native adoption prepares and verifies the candidate in the coordinator checkout. An epoch worktree can still contain the earlier base commit.
- Moving a coordinator invalidates path-bound run state. Prefer the primary checkout when it is clean and exclusively owned. An unrelated dirty primary requires an isolated checkout; archive its path-bound evidence before cleanup. Existing Chronis history and project Prime context must survive qualification and cleanup.

## Native methods

Update the shared plan, QA, and ship methods and bundled guide. Teach evidence requirements, not application-specific commands:

1. Bind acceptance to the supplied target when it is already explicit; probe only a missing capability.
2. Distinguish building from executing tests and exercising the actual application. After warming artifacts, establish that the application was rebuilt from the intended source before recording its digest.
3. Capture expected and observed exit status and separate streams, including negative cases. Treat timeout, signal, spawn failure, and lost process ownership as failures.
4. Read the candidate commit in the coordinator checkout returned by native adoption. Preserve the existing tracker and memory stores; keep path-bound archives separate from live state.

Keep installation additive and conflict-safe. Updating a previously generated method requires reviewing its replacement against the old generated version; do not silently overwrite customized skills.

## Supported command capture

Add `allstack check` as a generic helper beneath native skills. It executes an explicitly named executable through the existing Rust supervisor. Proposed interface:

```text
allstack check --expect-exit 4 --seconds 120 -- /absolute/executable arguments
allstack check --input prompt.txt --seconds 240 -- /absolute/host arguments
```

The command requires an ignored private `.allstack/` directory before writing evidence. It creates neither a delivery run nor a Chronis task. It does not open AllSource or Prime. A private receipt records the command, working directory, source revision and dirty state, executable digest, input digest when supplied, expected and actual exit, elapsed measurement, timeout/ownership outcome, and separate stdout/stderr. Preserve raw stream paths and digests; decoded JSON text must not be described as byte-exact output.

Only a matching normal exit with unchanged executable and retained evidence succeeds. An unexpected exit or other failure retains its receipt, returns a nonzero wrapper status, and names the receipt in the error. A digest binds the observed executable; it does not prove that executable was compiled from the recorded source. Native QA must establish that prerequisite separately.

Use one operation deadline, including metadata collection, with supervisor cleanup inside the existing CLI ceiling. Reuse existing process ownership and kill-then-reap behavior. No new dependency, per-application policy, retry loop, or host authentication mechanism is needed.

## Next real application change

Qualify the improved workflow on `all-source-os/all-source`: reject a zero Prime sync interval before store startup or background task creation. The isolated checkout starts from fetched main `897623e8` because the primary AllSource checkout has unrelated changes. Preserve those changes and use this owned checkout as coordinator throughout qualification.

The installed Prime 0.22.0 executable accepted `--sync-interval-ms 0`, panicked in its sync worker with `` `period` must be non-zero. ``, answered a genuine MCP `prime_stats` request with `sync.enabled: true`, and exited 0. Its SHA-256 is `3242e3341867d9cbae2b326db2ab0e0160db9afad124177fb407edb77aa0a349`. The isolated store, raw streams, and protocol output are retained privately under `.allstack/proof/allsource-native` in the founder-mode workspace. The destination was loopback port 9 and the key was synthetic; no hosted tenant was contacted.

Current AllSource source still accepts any `u64` interval in `apps/prime-mcp/src/main.rs` and passes it to both sync and email-ingest Tokio tickers. Inspect supported semantics before choosing the smallest validation seam. Explicit zero and `PRIME_SYNC_INTERVAL_MS=0` must produce an actionable nonzero configuration error before creating store files or contacting a remote. Preserve default 1000 ms and positive values, help/version behavior, existing local-only behavior, and genuine MCP tool responses. Prove valid sync against an isolated loopback Core fixture through the real MCP process. Build the current base as well as the candidate; the older installed binary is supporting evidence, not proof of current-main runtime behavior.

Claude must inspect and plan the actual application change through native `allstack-plan`; Codex must implement it; Claude must independently review and exercise the application through the updated native methods. Do not change live Prime configuration, hosted stores, release tags, or unrelated AllSource work.

## Verification and publication

- Exercise the actual installed `allstack check` boundary: successful exit, expected negative exit, unexpected exit, separate streams, spawn failure, timeout, executable binding, and no managed stores or tasks.
- Validate changed skills and existing installation conflict behavior. Run Allstack tests, formatting, and lint under the machine's exclusive compiler rule.
- Install the tested toolkit and verify its installed digest and bundled methods. Use normal authenticated Codex and Claude without model overrides.
- Freeze application acceptance before implementation; show the failing baseline and passing candidate against the actual built executables. Inspect actual native skill expansion, tool calls, diffs, and runtime receipts before accepting host claims.
- Run application gates against the exact single commit, publish one PR, and read back remote SHA, CI execution, AllSource history, Chronis completion, and source-linked Prime outcome.
- Keep an indexed qualification report in each affected repository. Include before/after behavior, reproducible commands, and limits. Raw host transcripts and private process receipts remain ignored.

## Chronis Git sync compatibility

AllSource versions `.chronis/sync/events.jsonl`, the portable export owned by `cn sync --git`. Full initialization and doctor must preserve and permit that exact path while continuing to reject tracked private `.chronis`, `.prime`, and `.allstack` state. Process tests cover both the preserved export and neighboring/private paths. Existing tracker engine data remains untouched.

## Preserved guarded base files

AllSource already tracks environment files, including examples and a production-named configuration. Native adoption must preserve these existing base files without copying their contents into Allstack artifacts. Snapshot manifests retain their Git blob identities instead. Git verifies both unchanged worktree contents and the expected base identity before capture or materialization; newly added, edited, or deleted guarded files remain rejected. Snapshot identity includes these references, and recovery validates them. Legacy snapshots without references retain their existing content digest. Process-backed integration tests cover restoration, artifact contents, rejection of changed files including `assume-unchanged`, and base changes invalidating materialization before writes.

## Authorization

The user authorized `feat/native-runtime-checks` in `all-source-os/allstack` and `fix/prime-sync-interval` in `all-source-os/all-source`, with one implementation PR in each repository. The user's machine rules forbid trunk pushes and take precedence over AllSource's checked-in direct-main default. Other AllSource PRs belong to separate sessions and remain untouched. Merge and deployment remain separate actions.
