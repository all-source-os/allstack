# Catalog bug-fix verification request

## Observed defect

Prime's recorder reconciles retries using only `<run>:<phase>`. A pre-existing matching key causes `Memory::record` to return success without checking whether the caller supplied the same workflow, source, or summary. That permits a conflicting retry to claim successful indexing even though retained memory describes another outcome.

The reproduction is `cargo +1.95 nextest run --locked --test prime_retry`. It uses actual embedded AllSource Prime, rather than a mocked graph. The identical-retry case must pass. Before repair, `conflicting_retry_cannot_claim_another_workflow_or_source` fails because a changed workflow is accepted. Retain the actual baseline command output with the workflow evidence.

## Requested repair

Repair `src/infrastructure/memory.rs` so a run/phase key binds one immutable workflow, source, summary, and run identity. Matching retries remain idempotent. Conflicting retries fail explicitly without adding or changing nodes. After a new write, read back the same complete binding before reporting success. Multiple nodes for the same key require reconciliation rather than selecting one arbitrarily.

Keep historical delivery records compatible: a missing `workflow_id` on an older node means `delivery` only. It must not reconcile as another workflow. Historical nodes remain recallable. Add focused coverage for legacy delivery and duplicate-key behavior if needed, in a separate integration test file. Preserve the frozen reproduction file `tests/prime_retry.rs` byte-for-byte.

Keep all prepared catalog implementation and unrelated source intact. Read the [catalog design](../plans/2026-10-24-workflow-catalog-design.md). Do not alter catalog definitions, method documents, frozen policies, or baseline evidence. Do not compile or create commits from the managed host call; the coordinator executes regression and complete gates afterward.

## Acceptance

- The retained baseline reproduces conflicting retry acceptance against actual embedded Prime.
- The same frozen two-test file passes after repair; all conflict combinations execute successfully.
- Missing legacy workflow metadata reconciles only with delivery; conflicting source or summary still fails.
- Duplicate keys and failed readback never produce a successful indexing claim.
- The complete toolkit suite, formatting, lint, and fixture checks pass on the exact published commit.
- Real Codex diagnosis and Claude repair run through selected `bugfix`, with immutable handoff and workflow-attributed Chronis/Prime readback.

## Retained baseline observation

[Baseline CI run 40143180893](https://github.com/all-source-os/allstack/actions/runs/40143180893) executed the frozen suite on Linux and macOS at `fe7423c8f7deb0f6f93228cc764fbd8756fd66eb`. Each platform reported 39 tests run: 38 passed, one failed, one skipped. The failure was:

```text
thread 'conflicting_retry_cannot_claim_another_workflow_or_source' panicked at tests/prime_retry.rs:57:9:
conflicting retry must fail: delivery, https://github.com/example/project/pull/1, repair verified
```

The assertion expected an error but received success for a changed workflow under an existing key. Formatting, dependency policy, and Clippy passed in the same run. Raw CI logs are retained privately; this excerpt contains the observed reproduction.
