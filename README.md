# Allstack

Plan, build, verify, and ship across coding agents, with durable history and evidence tied to the delivered change.

**Status: implementation preview.** The Rust CLI implements local run storage, task coordination, host invocation, immutable handoffs, verification, and GitHub publication. Source-level support is distinct from release proof: use the [acceptance scenarios](docs/reference/release-acceptance.md) to audit actual host journeys, interruptions, and delivered artifacts.

The [verification report](docs/reports/2026-10-24-cross-host-workflow-verification.md) records authenticated host execution, delivered forward-run evidence, reverse handoff observations, and supported limits. Private gate receipts remain readable through `allstack evidence <run>`; source-linked Prime context through `allstack recall <query>`.

Allstack embeds [AllSource Core](https://github.com/all-source-os/all-source) 0.26.1 for run history and Prime graph storage, and calls Chronis through `cn --toon` for task state. Prime recall is local lexical matching over delivery nodes; vector search and hosted synchronization are outside this preview. Coding agents remain responsible for model execution. The first release targets Codex CLI and Claude Code on one Unix machine, with an explicit handoff between them.

## First complete job

A developer plans a change in one host, continues implementation in another, resumes after an interruption, verifies the exact resulting commit, and publishes one reviewable pull request. The next host recovers the plan, workspace snapshot, evidence, and outstanding actions without copying a conversation transcript.

In this workflow, **ship means a verified commit pushed to a feature branch and an open pull request**. Merge and deployment are separate actions.

## Component boundaries

| Component | Owns |
| --- | --- |
| AllSource Core | Ordered workflow events and durable operation receipts |
| Chronis | Tasks, dependencies, claims, and task completion |
| Prime | Source-linked decisions, incidents, and relevant prior work |
| Allstack | Workflow transitions, handoff contracts, evidence binding, and reconciliation |
| Host adapters | Real `codex exec` and `claude --print` invocations with bounded process ownership |
| Git and forge | Source content, branches, commits, and pull requests |

Workflow methods and repository policies are versioned data. Generic mechanics belong in Rust. Optional browser, research, evaluation, and hosted-service integrations follow after the core workflow is proven.

## Use the preview

Install from a reviewed checkout with Rust 1.95 or later:

```sh
cargo install --path . --locked
allstack --help
allstack -C /path/to/target-repository init --check --profile rust
allstack -C /path/to/target-repository init --profile rust
allstack -C /path/to/target-repository doctor
```

`init` bundles workflow methods, indexed CLI documentation, repository-local Codex/Claude guidance, private-store ignores, and Chronis initialization. The explicit Rust profile requires `Cargo.toml` and `Cargo.lock`; an existing valid `--policy <path>` supports custom verification. Conflicting managed files are preserved and block writes. Reruns fill missing resources without replacing existing policies or repository instructions. Review initialized inputs before committing them.

The [installed-bootstrap verification report](docs/reports/2026-10-24-repository-bootstrap-verification.md) records fresh setup with actual Chronis and generated-policy execution while source-checkout reads were denied.

The [fresh-repository delivery report](docs/reports/2026-10-24-fresh-repository-delivery-verification.md) extends that proof through authenticated Codex → Claude Code → Codex handoffs, five executed acceptance tests, and a newly published [example PR](https://github.com/all-source-os/allstack-example/pull/1).

Starting a run needs an existing authorized feature branch, a GitHub `origin`, clean indexed inputs, and a frozen verification policy. Both coding hosts need their normal authentication and permissions. `doctor` reads setup and executable availability without opening mutable run stores; it does not authenticate hosts. Follow the [CLI guide](docs/reference/cli.md) for setup and the complete Codex → Claude Code → Codex sequence.

Each command holds an exclusive kernel lock on the local store. There is no persistent coordinator daemon or IPC service. Epoch checks fence Allstack-managed transitions; native tools used outside Allstack remain outside that boundary.

Snapshots require explicit `git add -- <path>` for new files. They reject symlinks and recognized sensitive filenames, and allow at most 4 MiB per file, 64 MiB total, and 10,000 files. These checks do not detect every possible secret. Keep private stores and process evidence under ignored `.allstack/`.

## Documentation

- [Documentation index](docs/README.md)
- [Cross-host workflow design](docs/plans/2026-10-24-cross-host-workflow-design.md)
- [Release acceptance scenarios](docs/reference/release-acceptance.md)
- [CLI and recovery guide](docs/reference/cli.md)
- [Delivery workflow data](workflows/delivery.toml)
- [Codex host guidance](hosts/codex/SKILL.md)
- [Claude Code host guidance](hosts/claude-code/SKILL.md)

Acceptance checkboxes remain open until retained evidence establishes each claim. Source tests and fixture definitions alone cannot establish actual cross-host behavior.

## Relationship to gstack

[gstack](https://github.com/garrytan/gstack) demonstrates opinionated planning, review, QA, and delivery workflows. Allstack uses that workflow shape as a reference and focuses its first release on durable cross-host continuation. Allstack's workflow text is original. Any future reuse must retain the applicable upstream and third-party notices.

## License

Apache-2.0. See [LICENSE](LICENSE).
