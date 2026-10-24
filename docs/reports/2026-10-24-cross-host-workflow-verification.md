# Cross-host workflow verification

Date: 2026-10-24. Target: public repository `all-source-os/allstack`, branch `feat/cross-host-workflow`, [implementation PR #2](https://github.com/all-source-os/allstack/pull/2).

## Observed forward journey

Run `9acd9993-3a70-4a87-b4fe-f62b96eb2cbb` used authenticated Codex CLI to plan the forward fixture, transferred its accepted snapshot and Chronis claim to Claude Code, and invoked Claude Code to implement `handoff_receipt`. A fresh Codex epoch recovered the source plan and candidate through separate CLI processes. No conversation transcript was transferred.

| Evidence | Observed result |
| --- | --- |
| Host sequence | Codex epoch 1 → Claude Code epoch 2 → Codex epoch 3 |
| Task | `t-3d5fb1`, read back as `done`, final claimant Codex epoch 3 |
| Delivered commit | `112a738e9d1debbaf34dcc5f06402c313492b8ab` |
| Runtime gate | 19 executed tests, 19 passed, zero skipped |
| Fixture gate | Two executed forward tests, two passed |
| Whitespace gate | `git diff --check HEAD^ HEAD`, exit 0 |
| Publication | Existing PR #2 and remote branch independently matched the verified commit |
| Component completion | Chronis completion and source-linked embedded Prime outcome confirmed |
| Cleanup | All three forward worktrees removed after their snapshots matched accepted history |

The forward destination already contained that commit from draft implementation publication. Allstack reconciled the destination and completed task/memory bookkeeping without another push or PR creation. The reverse exercise covers publication of a new candidate through the managed publisher.

Forward verification receipt digests:

- Runtime tests: `e4ad485d54c2b238519479bafa50e84d6893a006c7bc4cc9d88ba0105e68fffd`.
- Forward fixture: `a56d0f6aa566c51658bf9412d9e20f53165864144a57b5ca69336a7e4c3a5133`.
- Whitespace: `0855960220f15bd59fc22b2ab38462ca0839a480bed8b506d24206a8bbf0f53e`.

## Reverse exercise and interruption

Run `750de2d1-ec0b-4327-be96-4f0e1ba5c5f5` starts from the forward-delivered commit, with Claude Code owning epoch 1 and Chronis task `t-649659`. Its request implements whitespace normalization by delegating to the strict transfer function.

The real planning caller was terminated with SIGTERM. Its host process receipt recorded `owner_lost: true`, no successful exit code, and an elapsed 2,927 ms. Reopening AllSource returned the durable `host_plan` intent. Recovery retained the last checkpoint, cleared uncertainty, and left planning necessary. It did not replay the uncertain model call. No remaining process argv contained this run's identity after interruption.

A subsequent planning invocation completed a plan but attempted an unavailable shell listing tool. The adapter refused to accept that result. Claude's exposed tool set now limits planning to `Read`, `Glob`, and `Grep`; build adds editing and explicitly permitted Git commands. Repository hooks and native permission modes remain enabled.

The final reverse host transitions, test receipts, exact delivered SHA, and destination confirmation are available through the run's `history` and `evidence` commands. The PR description records the final SHA after publication; a document cannot contain its own Git commit hash.

After recovery, the narrowed Claude planning invocation completed without permission denials. Codex epoch 2 received a distinct restored worktree and implemented `normalized_receipt` by trimming inputs and delegating to the strict function. Claude epoch 3 received that resulting snapshot. The frozen reverse policy requires the runtime regression suite, all four forward/reverse fixture tests, and the candidate whitespace gate before delivery.

## Exercised refusal and isolation paths

- A second real CLI command was refused while Claude held the coordinator's kernel lock.
- An outgoing Codex epoch 1 checkpoint was refused after ownership advanced to Claude epoch 2.
- Editing the outgoing worktree did not change the incoming candidate. The probe edit was restored before cleanup.
- Changing `RUSTFLAGS` caused managed shipping to fail with `verification stale`, before publication.
- Competing external Cargo builds blocked required compilation. Verification retained its pending state and later succeeded after the compiler slot became idle; no peer process was terminated.
- Trailing whitespace in a generated plan stopped commit preparation. Explicit checkpoint repair retained the known checkout snapshot and enabled a later verification attempt.
- Process tests exercised deadline termination and owner-pipe loss using real children. AllSource tests reopened real embedded storage, rejected a competing writer, rejected stale writes, and detected a forged history gap. Artifact tests rejected corrupted bytes.
- Application fault tests covered a lost task-transfer acknowledgement, task completion failure after publication, Prime indexing outage, stale candidate receipts, and re-verification. These are injected-port tests, distinct from the authenticated host journeys.

## Executable and component provenance

| Component | Version or digest |
| --- | --- |
| AllSource Core and embedded Prime | Core `0.26.1`, `embedded` and `prime` features |
| Chronis | `cn 0.9.1` |
| Codex | `codex-cli 0.163.0` |
| Claude Code | `2.1.310` |
| Git | `2.46.1` |
| GitHub CLI | `2.101.0` |
| Verification Rust toolchain | `1.95` |
| Workflow digest | `d38b06fde2ccfb96e1f721579659461edc7b3c1fcdd0461fd43b842f4c09b169` |
| Forward verification policy | `8e53ad4d053288f76a75f78042aa4b9c0353f47ec354ba39a832a8a64e0f2986` |
| Reverse verification policy | `b66b39ff4aaa3ed20bf1efad5def2d23e34c986f0ba93d9a5ec859bdace73004` |
| Repaired adapter executable SHA-256 | `a61bc48cc407306e2e4f586f112a88285b1685f63a14db8a7634d42cb5ef1593` |
| CI bootstrap executable SHA-256 | `baa31b0bca4ca08dabd053fc0be708498d8f71821b18018fd23eb782c333ac7f` |

[CI run 40114594845](https://github.com/all-source-os/allstack/actions/runs/40114594845) executed 19 tests on both Linux and macOS, with formatting, Clippy, advisory/license/source checks passing. Its macOS executable was built from synthetic merge commit `254761849251daaf3fc8a4793a494bfd56be3cf9`. The merge's tree and the reviewed branch's tree both equal `b783b5bcd8ee3fa9b72eb04db141629292211b8a`; artifact bytes matched the uploaded SHA-256. Exact-commit local gates subsequently passed on the branch commit itself.

The locked graph contains 258 packages, including Allstack: 248 distinct names and eight names with multiple resolved versions. The embedded Core dependency supplies the Arrow/Parquet stack and those transitive version splits. Direct TOML and SHA-2 dependencies share Core's resolved versions. Defaults on Core remain disabled; vector inference dependencies are excluded.

## Inspect and reproduce

Follow the [CLI guide](../reference/cli.md), using the [forward](../reference/handoff-forward-request.md) and [reverse](../reference/handoff-reverse-request.md) requests. In the original local run store:

```sh
allstack history 9acd9993-3a70-4a87-b4fe-f62b96eb2cbb
allstack evidence 9acd9993-3a70-4a87-b4fe-f62b96eb2cbb
allstack history 750de2d1-ec0b-4327-be96-4f0e1ba5c5f5
allstack evidence 750de2d1-ec0b-4327-be96-4f0e1ba5c5f5
allstack recall 'verified commit remote task completion'
```

These commands read component state through Allstack and its embedded engines. The private store and unredacted process output are excluded from Git. A fresh clone does not contain the original private store; it can run the versioned fixtures and create its own runs.

GitHub reported `main.protected = false` during verification. CI results provide review visibility and do not block a direct merge. The managed publisher enforces its frozen verification policy before delivery; the repository enables rebase merges and disables merge commits and squash merges.

## Boundary of this evidence

This preview covers sequential hosts on one Unix machine, local embedded storage, lexical Prime recall, and GitHub feature-branch publication. Epoch numbers fence managed transitions; they do not authenticate a local user or sandbox native host tools outside Allstack. Selected environment fingerprints are not hermetic builds. Filename exclusions are not a complete secret scanner.

The caller-loss test does not establish behavior after SIGKILL of every guardian at once, machine power loss, arbitrary native-plugin daemonization, or cross-machine transfer. Merge and deployment were not performed. Unchecked [release acceptance criteria](../reference/release-acceptance.md) remain unproven rather than being inferred from passing tests.
