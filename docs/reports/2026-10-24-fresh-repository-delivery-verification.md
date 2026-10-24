# Fresh repository authenticated delivery verification

Date: 2026-10-24. [Example repository](https://github.com/all-source-os/allstack-example), [delivered proof PR #1](https://github.com/all-source-os/allstack-example/pull/1), and [toolkit PR #3](https://github.com/all-source-os/allstack/pull/3).

## Delivered result

An installed Allstack executable initialized a new Rust repository, invoked authenticated Codex planning, handed the retained plan and immutable source to authenticated Claude Code, recovered the candidate in a fresh Codex epoch, verified its exact commit, pushed a new remote branch, and created an open proof PR. Chronis completion and an embedded Prime outcome were subsequently read back through their own tools.

The delivered implementation accepts exactly the two supported host transfers and rejects unknown, empty, unchanged, whitespace-padded, and differently cased names. Claude changed only `src/lib.rs`; the five frozen integration tests remained byte-identical. The initial fixture executed four passing tests and one failing valid-transfer test. The delivered candidate executed all five successfully.

| Evidence | Observed result |
| --- | --- |
| Installed executable SHA-256 | `4e794ece7b2cd8ec06a99e8914cf811af6d3808be6081167bdb9685159d4d3d7` |
| Managed run | `9f981bf4-cb9d-477e-8c5d-8864e1296834` |
| Host sequence | Codex epoch 1 → Claude Code epoch 2 → Codex epoch 3 |
| Example branch | `feat/handoff-receipt` |
| Exact verified and published commit | `ccb3f4fe96d7b8c3dda7c4e8f39a0f1aa381a540` |
| PR destination | `all-source-os/allstack-example#1`, open, one commit, head equal to verified SHA |
| Chronis project task | `t-f05a7f`, `done`, claimant `allstack:9f981bf4-cb9d-477e-8c5d-8864e1296834:codex:3` |
| AllSource history | 22 validated events; reopened final revision 22, stage `shipped`, no pending operation |
| Prime outcome | Delivery node `55fcbdde-dc83-4d85-96bc-c95389c8da8f`, key `<run>:shipped`, source equal to proof PR URL |
| Cleanup | All three run-owned detached worktrees removed; example primary checkout clean |

The executable is the same installed artifact qualified in the [bootstrap report](2026-10-24-repository-bootstrap-verification.md). This journey required no toolkit runtime change. Its evidence and reproduction request are versioned on toolkit PR #3.

## Initialization and authenticated execution

The example began as a new local Git repository with a dependency-free Cargo package, lockfile, stub function, and five acceptance tests. `init --check --profile rust` reported nine planned resources with no conflicts and created neither project store. Applying `init --profile rust` created those resources and initialized the target's actual Chronis store. Doctor returned `setup_ready: true`, `executables_ready: true`, and `chronis: readable`.

The public GitHub repository started with a server-generated README/license commit. Its default `main` was fetched, and the explicitly authorized feature branch retained the local scaffold. No trunk push occurred. The forge was configured and read back as rebase-only.

Codex CLI `0.163.0` reported a ChatGPT login. Claude Code `2.1.310` reported `loggedIn: true`, `authMethod: claude.ai`, and `apiProvider: firstParty`. Actual model calls then completed through the installed coordinator:

| Host receipt | Retained observation |
| --- | --- |
| Codex planning | Exit 0, `turn.completed`, 15 recorded command-tool completions, 75,325 ms |
| Claude implementation | Exit 0, `subtype: success`, `is_error: false`, zero permission denials, 19,163 ms |
| Claude receipt model label | `claude-opus-4-8` |

Codex planning receipt SHA-256: `f3d99fe919328525d64282d35b131d3b2f49da61a11399adb9d923eeaa1b0039`. Claude implementation receipt SHA-256: `05b8b0f1f74f121f846f5e58750026ecddaa3cf026499865a2852446455a2ba1`. Raw transcripts and account identity fields remain private.

The initial Codex plan recorded that the custom installation prefix was absent from its `PATH`. The prefix's `bin` directory was added to the inherited environment before handoff and build. The accepted plan remained unchanged. Operators using a custom prefix must expose that executable directory to their hosts. Rust 1.95 and the selected environment were then held constant through verification and shipping.

## Handoff, verification, and publication

The plan was indexed at `docs/plans/allstack-<run>.md` and retained with digest `0dceaacea7ddf27ccbde2acc2a6b8d545bda74e668763f585b62db97a9cfeedf`. The first handoff restored a distinct Claude worktree from checkpoint `d6f0388aaa06bd657a2069c94ff423cbbf85353066c31e92e66ae4b93953ca45` and advanced the Chronis claimant to epoch 2.

After Claude edited the implementation, its checkpoint was `2493f542f03c42be47317a2a12cb7ea203b5e923922037322acae13b3c5fe639`. A second handoff restored that candidate in Codex epoch 3. A separate `resume` process returned authoritative request, plan, run state, and source-linked Prime recall. The incoming source was inspected, and its test-file digest was compared with the original frozen fixture.

The unchanged generated policy digest was `9b39285fd803258d73450d1a2e1fcccd2a8f591dc53c4a684ec57431b80358a3`. Managed verification created the owned one-commit candidate and executed:

| Gate | Actual result | Retained receipt digest |
| --- | --- | --- |
| `cargo fmt --check` | Exit 0 | `5d1fd303ccd458b9607b961aae6041a91ceebfe43b0dfdea9d8db7b822d50376` |
| `cargo test --locked` | Five integration tests passed, zero failed | `f6cb2cd058fe2b3d0b3c0af457bc755c2ff03d5c4519dd3c5b99e39e118940d8` |
| `git diff --check` | Exit 0 | `0855960220f15bd59fc22b2ab38462ca0839a480bed8b506d24206a8bbf0f53e` |

The test output also contained zero-test library/doc units. The positive five-test integration summary supplied execution proof. The frozen test-file SHA-256 was `6dd68840773b48a07aafa1f219e68eb4347880a69d19e6ae56c1c2a8b55ec46f`. The delivered implementation SHA-256 was `e060c77c059cdd3fe70192ac0a4a0494ab3afda0d59175a60d18bd4fbc4885da`.

Managed shipping performed a real new-branch push and real PR creation. Their retained process receipt digests were `234f66e65041b354de9e81e1869a29fdf7a0f999e262cb79470b38266fd2e21e` and `73d08c1685cc866f44ee6938f408e8bffa5294724cbfa7eaf03aca0b5a4224db`, both exit 0. Independent GitHub API reads confirmed the actual source bytes, open PR, one commit, and head SHA; `git ls-remote` confirmed the same branch SHA.

Chronis was read from the example repository's project store, rather than the toolkit tracker. Embedded Prime was read through the example's `allstack recall` command, rather than the global Prime MCP store. Reopened run state confirmed `publication.task_completed: true` and `publication.memory_indexed: true`.

## Inspect and reproduce

Use the [fresh-repository request](../reference/fresh-repository-delivery-request.md), an installed executable on the inherited `PATH`, normal host authentication, and an explicitly authorized repository/feature branch. Follow the [CLI guide](../reference/cli.md). The [delivered example plan](https://github.com/all-source-os/allstack-example/blob/ccb3f4fe96d7b8c3dda7c4e8f39a0f1aa381a540/docs/plans/allstack-9f981bf4-cb9d-477e-8c5d-8864e1296834.md) retains the actual Codex response.

With the retained example checkout and private stores:

```sh
allstack -C /path/to/allstack-example history 9f981bf4-cb9d-477e-8c5d-8864e1296834
allstack -C /path/to/allstack-example evidence 9f981bf4-cb9d-477e-8c5d-8864e1296834
allstack -C /path/to/allstack-example recall 'verified commit'
cn show t-f05a7f --toon
```

Run the final Chronis command from the example checkout. A fresh clone contains public source and the indexed plan, while original private stores remain local.

This proves a fresh installed setup through authenticated model execution, immutable host handoffs, final-commit verification, new remote publication, task completion, and source-linked outcome readback. Scope remains sequential hosts on one Unix machine. Merge, deployment, hosted synchronization, and fresh-fixture crash injection were not exercised.
