# First-release acceptance scenarios

Status: implementation-preview release gate. Every checkbox remains open until retained evidence establishes it. Source-level contracts, passing unit tests, and journey definitions do not by themselves establish authenticated host behavior.

## Complete job

- [ ] Fresh installation works without an AllSource hosted account and reports actual component versions, storage scope, sync state, host capabilities, and missing prerequisites.
- [ ] Codex CLI plans an accepted repository change, Claude Code builds it, and Codex CLI resumes verification and publication after interruption.
- [ ] The reverse journey, Claude Code → Codex CLI → Claude Code, completes through the same contracts.
- [ ] Both journeys use the actual supported hosts and tools, not manually fabricated lifecycle events.
- [ ] A real end-to-end repository flow runs, required tests execute, the exact final commit passes required gates, and the resulting one-commit PR points to that SHA.
- [ ] Task state is read back from Chronis after publication, and Prime contains a source-linked outcome without becoming publication authority.

## Handoff and recovery

- [ ] Handoff retains tracked changes, deliberately included untracked inputs, required modes, and every referenced artifact before acknowledging success.
- [ ] Incoming host restores a distinct worktree and verifies the snapshot; outgoing host edits cannot change the accepted candidate.
- [ ] Stale host epochs cannot append transitions or invoke managed publication.
- [ ] Crash before checkpoint reports the last durable checkpoint and identifies any newer worktree content requiring inspection.
- [ ] Missing artifacts, changed bases, path escapes, mismatched hashes, and unavailable setup inputs block mutation with actionable errors.
- [ ] Partial Chronis ownership transfer blocks execution until actual claimant state is reconciled.

## Storage and component consistency

- [ ] Actual AllSource writes survive process restart with complete, validated sequence and idempotent operation recovery.
- [ ] Competing CLI coordinators cannot mutate one local run store; the per-command kernel lock and stale-epoch checks are exercised across real processes.
- [ ] Corrupt, truncated, reordered, duplicated, or partially retained history cannot produce a verified stage.
- [ ] Reusing an operation ID with different input fails; lost append acknowledgements recover by reading the store.
- [ ] Crash between run state and Chronis/Prime update leaves recoverable pending work rather than contradictory completion claims.
- [ ] Uncertain task or memory creation is reconciled without assuming a retry is idempotent.
- [ ] Prime outage or stale recall cannot fabricate state, permission, or a successful check.

## Verification and publication

- [ ] A test command selecting zero cases cannot satisfy a required test gate.
- [ ] Candidate changes, workflow changes, policy changes, a rewritten commit, and relevant environment changes invalidate affected receipts.
- [ ] Final verification runs on the commit actually pushed and detects dirty output from checks.
- [ ] Missing branch/commit/publication authority is requested against the prepared action; previously granted applicable authority is honored.
- [ ] Trunk pushes, dead branches, foreign-history rewrites, hook bypasses, and a second open PR are refused under repository policy.
- [ ] Remote ref and PR head are read back independently before the run records shipped.
- [ ] Lost push or PR-create responses trigger destination reconciliation and do not produce duplicate publication.
- [ ] A later Chronis or Prime update failure does not cause a second push or PR create.
- [ ] Merge and deployment remain distinct from shipped state.

## Process and privacy boundary

- [ ] Owned commands time out within 300 seconds, then terminate and reap their owned process tree.
- [ ] Caller or guardian death cannot leak owned children; process-level evidence checks the owner-pipe and kill/reap invariant.
- [ ] One local build runs at a time; external competing builds are observed without terminating another session's work.
- [ ] Runtime stores, unredacted transcripts, source snapshots, credentials, and live capabilities stay out of the public repository.
- [ ] Host support documentation distinguishes managed-action enforcement, advisory guidance, and unsupported operations.

## Evidence required for a release

Retain the release binary/package digests, host and component versions, workflow digest, synthetic fixture revision, complete run IDs, exact delivered commit SHAs, actual check outputs, interruption points, and reconciled publication receipts. Publish an indexed evidence document containing only material intentionally suitable for public distribution.

Passing scaffold validation establishes only that documentation links and example workflow data are consistent. It cannot satisfy any runtime checkbox above.

## Preview coverage boundary

The runtime uses AllSource Core 0.26.1 embedded storage, Chronis through `cn --toon`, and an embedded Prime graph with lexical recall. Each CLI command owns an exclusive kernel lock; no persistent daemon or authenticated IPC service exists. Epoch fencing covers managed transitions and publication. Native host tools outside Allstack remain outside that enforcement boundary.

Release evidence must cover both host directions, not infer one from the other. Fixture placeholders are specifications awaiting actual host execution. A fixture name or manually written lifecycle event cannot satisfy the real-host criteria.

The initial scope is one Unix machine, sequential detached worktrees, `github.com` publication, and positive Cargo/nextest test execution counts. Snapshots require explicitly staged new files and reject symlinks and recognized sensitive filenames, with caps of 4 MiB per file, 64 MiB total, and 10,000 files. These exclusions are not a complete secret scanner. Merge, deployment, vector retrieval, hosted sync, and cross-machine import/export are outside the preview.
