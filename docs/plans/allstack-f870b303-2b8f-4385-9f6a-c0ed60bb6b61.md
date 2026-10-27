# Native evidence checkpoints and Chronis qualification

## Complete user job

A developer asks for a real application repair through native skills. Claude reads the installed planning method and current source, Codex implements the accepted change, Claude independently reviews and exercises the application, and Allstack publishes the verified commit through one PR per repository. AllSource retains delivery history, Chronis owns claims and completion, and project Prime returns source-linked plan and outcome records. Evidence remains readable after owned worktrees are removed.

## Observed constraints

Merged [AllSource PR #364](https://github.com/all-source-os/all-source/pull/364) records existing-consumer compatibility, deployed-source provenance, actual host calls, and a failed HTTP observation whose cause could not be established because its body was missing. Its [verification report](https://github.com/all-source-os/all-source/blob/fabca9d0e420578774f0d80eecaa8480f15e8706/docs/reports/2026-10-26-managed-key-lifecycle-verification.md) separates synthetic hosted sync from the later existing-key admission proof. These are bounded observations, not evidence that every tenant or deployment succeeded.

The current native interface already has seven methods, bounded `allstack check` receipts, source-linked memory, native adoption, and exact-commit shipping. The missing guidance concerns evidence decisions around those existing capabilities. No new engine behavior or catalog entry is needed for this correction.

## Method changes

| Method | Decision added |
| --- | --- |
| `allstack-plan` | Read producer revision/count semantics and representative retained history; include an existing consumer's next operation. Compare frozen gates with changed components and name supplemental coverage. |
| `allstack-review` | Trace legacy compatibility and the complete existing-consumer path. Successful authentication alone does not establish application success. |
| `allstack-qa` | Preserve safely redacted failed HTTP responses and later successes separately. Bind host execution to actual calls; distinguish running-image proof from final-commit checks. |
| `allstack-handoff` | Ask the receiving native host to read its installed method and current source; retain calls and results instead of accepting its summary as execution evidence. |
| `allstack-ship` | Archive epoch receipts and raw streams before shipping removes accepted worktrees; verify lengths/digests, read back tracker/memory through their engines, and preserve state before owned cleanup. |

The installed native guide carries the packaging implications. Skill bodies remain identical across Codex and Claude apart from existing host overlays. The seven names, invocation modes, permission boundaries, CLI contract and delivery state machine remain stable.

## Alternatives

1. **Update existing method data, then qualify a real change.** This uses the established packaging and runtime mechanisms. Recommended because the observed gaps concern what an agent checks and reports.
2. Add an orchestration layer that forces a new receipt schema or deployment state machine. This would expand the product before the method-level repair is exercised; current receipts already retain the necessary streams and identities.
3. Add a retrospective only. This preserves evidence but leaves incoming native hosts without the operational guidance at the relevant decision point.

## Real application target: Chronis TOON integrity

Current AllSource main `e1416e58eb485bce243f1cc728160f1302289e1c` contains Chronis 0.9.1. `apps/chronis/src/presentation/toon.rs` escapes pipes and newlines in titles but leaves backslashes ambiguous and prints `claimed_by` without escaping. An isolated real CLI reproduction with installed `cn 0.9.1` demonstrated two failures:

- A title containing literal backslash plus `n` and a title containing an actual newline produce identical rendered title values.
- A custom `CN_AGENT_ID` containing a pipe and newline inserts an extra table column and a `status:done`-looking detail line, although actual task state remains in progress.

Private raw commands and outputs are retained under the founder-mode ignored `.allstack/proof/native-lessons-2026-10-27/` directory. Installed-binary reproduction is supporting evidence; qualification must also rebuild and exercise the current baseline source.

Repair the TOON presentation boundary. Preserve exact stored task/claim data and the JSON representation; do not sanitize writes or change task lifecycle rules. Define unambiguous escaping for backslashes, pipes and line controls, apply it consistently to free-text values, and verify ordinary records remain stable. Inspect downstream readers before fixing the contract. No new dependency, hosted credential, production deployment or store migration is required.

Authenticated Claude session `49403bff-e4f2-4447-94ef-9045edaccfd1` read the installed native planning skill, actual baseline receipt and current formatter/callers. Its 12 observed file-tool calls support the proposed repair site; it performed no application execution. Native receipt `1f00b611-0ee9-4c99-b662-f6572fbce943` records a normal zero exit, and all archived raw streams matched their recorded lengths and digests. Its source inspection does not replace the required current-baseline build.

The acceptance parser must scan escapes once and count only unescaped field delimiters. A plain `split('|')` would reject valid escaped pipes, and chained reverse substitutions can collapse literal escape sequences. Assert round trips through a single-pass decoder used by the acceptance harness. Preserve the existing comma-separated dependency representation for generated IDs; changing that subformat is outside this defect's observed path.

Expected application files are the existing Chronis TOON encoder, focused sibling/integration tests, and an indexed application plan/report. Final file selection follows source-grounded native planning. Do not edit unrelated primary-checkout changes or another session's open deployment PR.

## Qualification

1. Validate changed skills with the bundled skill validator. Run existing native packaging/conflict checks and applicable Allstack gates; do not add tests that merely match instruction wording.
2. Install the tested Allstack executable and read back its digest, bundled methods, and generated methods for both hosts. Replace only resources verified against their previous generated form; preserve local customizations.
3. Freeze Chronis acceptance before implementation. Build baseline and candidate from their actual source, with one machine compiler at a time and commands bounded below 300 seconds. Warm caches do not establish artifact provenance.
4. Through the real `cn` CLI in an isolated workspace, create distinct literal/control-character titles, claim using a delimiter-bearing agent identity, reopen with separate processes, list/show, complete, and read back persisted state through Chronis. Assert unambiguous output, stable field/row counts using an escape-aware parser, no injected status field, preserved JSON data, and unchanged ordinary output. Include invalid lifecycle transitions with explicit expected exits.
5. Use authenticated Codex and Claude with their normal configured models and permission modes. Retain the receiving host's actual skill-read calls, source reads, test commands, output and executed case counts. A source review alone cannot satisfy runtime QA.
6. Adopt the native application candidate with a frozen policy covering Chronis, verify the single commit, and ship its one PR. Read back remote branch/PR head, final CI, Chronis completion, and project Prime plan/review/QA outcomes.
7. Preserve receipt/raw-stream paths, byte lengths and digests outside disposable epoch worktrees before shipment. After publication and store readback, archive isolated coordinator state and remove only owned idle worktrees/builds.

The Allstack report must distinguish baseline failure, candidate behavior, actual work performed by each host, installed toolkit provenance, policy coverage, and material limits. Qualification covers sequential real hosts on this machine; it does not imply a second physical machine, hosted sync on a new revision, or deployment.

## Branch and publication prerequisites

The user authorized `feat/native-evidence-checkpoints` in Allstack and `fix/chronis-toon-escaping` in AllSource, with commits and one PR each. Both branches start from their current main. The isolated AllSource coordinator preserves unrelated changes in the primary checkout and remains separate from the deployment-gate PR owned elsewhere.

Supplied machine policy requires explicit new-branch authorization and prohibits trunk pushes. Commit and PR publication proceed only on those authorized live branches. Merge and deployment remain separate actions.
