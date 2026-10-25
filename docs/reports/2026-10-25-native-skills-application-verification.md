# Native skills and application verification

Date: 2026-10-25. Publication target: [Allstack PR #5](https://github.com/all-source-os/allstack/pull/5). Scope: original native methods, installed Codex/Claude skills, standalone source-linked recall, and a real expense-report change carried through durable delivery.

## Product behavior

The primary interface is `allstack-shape`, `allstack-plan`, `allstack-review`, `allstack-qa`, `allstack-ship`, `allstack-recall`, and `allstack-handoff` inside the coding host. Shared method sources are packaged with small host capability overlays. Independent planning, review, QA, and recall require no delivery run or Chronis task.

`allstack init --skills-only` installs that interface without a project profile or Chronis initialization. `skills` works outside Git and opens no stores. `remember` retains a bounded summary and document digest in local Prime; `recall` retrieves standalone knowledge and existing delivery outcomes. `adopt` enrolls an existing candidate and accepted plan without repeating model work. `current` discovers one active job through validated AllSource histories and refuses ambiguity.

Managed delivery keeps immutable scope, expected revisions, Chronis claims, explicit handoff, process ownership, exact-commit gates, and remote/PR readback. No new workflow ID or weaker publication path was introduced.

## Installed artifact

- Final isolated CLI SHA-256: `9d18314eff02b59ef78e12507db46c85f24821351070c71c935f8fc9f7b11d34`.
- Skills-only setup installed seven native skills for each host, reran unchanged, and created no Allstack, Chronis, or Prime stores before host execution.
- A real installation produced by the previous verified CLI (`576a673ebc9975e09e50f06445131d1ebff2ef7ce0b3df60aa751217bc19d969`) upgraded through the final CLI. Fifteen existing resources, including the policy, remained byte-identical; the documentation index gained its native guide link. Full setup contains the seven native skills plus the two existing delivery/catalog skills per host.
- Upgrade and idempotent rerun succeeded while macOS denied reads of both source checkouts. Final bundled skill discovery also succeeded outside Git.

The isolated executable was installed from the built binary. The global Allstack installation was not replaced. Resource and executable manifests remain with the private qualification evidence.

## Authenticated native work

All hosts used their normal authentication and configured models. No model override or permission bypass was used. The Rust qualification harness uses the production owner-pipe supervisor with one bounded deadline and retained process receipts.

| Operation | Observed result | Receipt SHA-256 |
| --- | --- | --- |
| Claude native shaping/planning | Read both installed methods and application source; returned the engineering plan; exit 0, 221985 ms | `be6cc7a969b2cc5c61938a6d593a9df7eec5bff4ea1dbfc65a5c095004433cbb` |
| Codex native recall | Discovered `allstack-recall`, called project `allstack recall`, opened the source, and verified its digest; exit 0, 72702 ms | `862a047de050623fe5a815606e770471cf342771d1f36117ea2f862c3d5b16df` |
| Claude native review/QA | Invoked `/allstack-review` and loaded `allstack-qa` through the native Skill tool; four actual CLI commands; exit 0, 110933 ms | `d52a1ad508dc1cf20ec527f6bb948a3e3a5ab4d03b37d87d6a6752b19062d690` |

The native review project remained without Chronis or Allstack delivery events. Its Prime decision node `d7299784-9196-46c2-8cd9-4a93af3a63c3` retained source `docs/plans/2026-10-25-spendcheck-category-plan.md` with digest `eee470beb8360bce1188988ae455654e8480b8a387cb8bc9c666b9adb41d0ad2`. The fresh Codex session's actual query returned that node, and its actual SHA command matched the document bytes.

Initial probe launches failed because the command environment omitted the installed Claude directory. Retained supervisor stderr identified the spawn failure; the successful probes used the observed executable and explicit PATH. The constrained planning probe also attempted one unavailable Write tool. Its tool result reported that Write was disabled, the attempted file does not exist, and the plan was returned for indexing. Both completed Claude responses reported zero permission denials. These observations are retained rather than counted as successful writes.

## Complete application change

[Spendcheck](../../examples/spendcheck) summarizes strict CSV expense records in exact minor units and keeps currencies separate. The change adds `--category <name>` with exact, case-sensitive matching. Every row is validated before filtering, including excluded rows.

Before implementation, both baseline tests passed; category acceptance executed six tests with two passing and four failing. After real Codex implementation, the unchanged two baseline tests and six category tests all passed. Their source digests remained fixed:

| Frozen input | SHA-256 |
| --- | --- |
| `tests/baseline.rs` | `36278b65d0cec35d8f5b277f02453528ed11c0552a9db6786123b7e97c11c393` |
| `tests/category.rs` | `0b7baddea9c2a0728117fafbddcf3537e41d9b8143ed988ceac34793dc3ddafc` |
| `expenses.csv` | `3089824068939b1858f768f14f27cf557118183833aa0843da3802c0ff1bfde8` |

The inspected Codex manifest changed exactly `src/lib.rs`, `src/main.rs`, and the example README. The managed host receipt recorded exit 0 and 101721 ms. Its actual commands read the recall method and retrieved project context, then checked the accepted plan and frozen input hashes.

Claude's actual QA tool results matched these outcomes. The coordinator independently repeated the four executable cases and inspected separate exit status, stdout, and stderr:

| Input | Exit | stdout | stderr |
| --- | --- | --- | --- |
| Unfiltered sample | 0 | `records: 4`, `EUR 14.50`, `USD 7.00` | empty |
| `--category travel` | 0 | `records: 3`, `EUR 10.00`, `USD 7.00` | empty |
| `--category rent` | 0 | `records: 0` | empty |
| Malformed excluded record | 1 | empty | `invalid amount at line 3` |

QA executable digest: `d4b396482b8f65e8fae85178f324d568db2da5571bab1586026d57a11fdb9f51`. A source manifest proved the standalone review copy matched the managed candidate. The native review reported no actionable findings; this does not establish absence of all defects. No browser verification is claimed.

## Durable delivery and final verification

AllSource run `9dc4c81f-73ca-4b92-89a2-be3c2ca44deb` adopted the native plan and prepared candidate. Claude owned epoch 1, real Codex implemented at epoch 2, and the candidate returned to Claude at epoch 3. Chronis task `t-16f532` claims were read back at both handoffs. The coordinator Prime decision node is `7f70950b-007a-4952-a588-768c4d933b04`.

The frozen [verification policy](../../workflows/native-gates.toml), digest `82cce6955bfc9dcd50b850f5905245400b2752504e14c5bdf524f4a8b2c7c387`, executes runtime/native tests, application tests, existing host fixtures, formatting, lint, and source checks. Its final receipts bind the candidate, environment, installed CLI, and exact single commit. Shipping must confirm that same commit at the remote branch and PR head before task completion and the Prime outcome.

The complete policy executed 53 runtime/native tests, eight application tests, and four existing host-fixture tests successfully. Both actual-Chronis qualification cases are excluded from the portable suite; the staged-deletion qualification ran separately and passed one executed case. Formatting, lint, and source checks also passed. The final publication receipt identifies the exact tested commit.

Native adoption also has an explicit staged-deletion regression qualification. The pre-repair native CLI (`962c61e52fa24860b45559c47043fc91d8cf1997d703288c1dae39e0f57d9db0`) failed with `pathspec 'obsolete.md' did not match any files`; the repaired CLI passed the same qualification with actual Git index changes, signed commits, real Chronis, and executed project gates. Only remote discovery/forge reads were protocol fixtures. The coordinator now stages unstaged paths and separately checks the cached candidate, preserving files already staged for deletion. The qualification executes separately because it requires configured Chronis and Git identity; CI reports it as skipped rather than treating it as a live success.

Use the run's validated history/evidence exports and PR head/checks to audit final publication. Private proof includes native tool ledgers, actual process results, source/resource manifests, before/after test logs, and engine readbacks. Linux/macOS CI also executes the application suite. A test runner's empty unit targets are not counted as executed acceptance tests.

Supported limits remain sequential hosts on one machine, local lexical Prime context, and native tools governed by host policy outside managed calls. Hosted memory synchronization, merge, deployment, and browser QA are outside this proof.

The owner-loss test waits for a complete newline-terminated PID before revoking the owner pipe. Its cleanup closes that pipe and reaps the supervisor on failure. The child-reaped assertion and original deadlines remain intact.
