# Allstack

Opinionated product and engineering skills for Codex and Claude Code, backed by durable project memory and delivery evidence.

**Status: implementation preview.** Invoke native skills to shape an idea, plan a change, review an existing branch, exercise an application, or ship authorized work. AllSource, Chronis, and Prime support durable continuation underneath. Source support and real-host acceptance are tracked separately in the [documentation](docs/README.md).

The [verification report](docs/reports/2026-10-24-cross-host-workflow-verification.md) records authenticated host execution, delivered forward-run evidence, reverse handoff observations, and supported limits. Private gate receipts remain readable through `allstack evidence <run>`; source-linked Prime context through `allstack recall <query>`.

Allstack embeds [AllSource Core](https://github.com/all-source-os/all-source) 0.26.1 for run history and Prime graph storage, and calls Chronis through `cn --toon` for task state. Prime recall is local lexical matching over delivery nodes; vector search and hosted synchronization are outside this preview. Coding agents remain responsible for model execution. The first release targets Codex CLI and Claude Code on one Unix machine, with an explicit handoff between them.

## First complete job

A developer shapes an application change, reviews its architecture, implements it, checks concrete failure risks, exercises the real user flow, and publishes one verified PR. Each specialist also works independently. An explicit host change preserves the accepted plan, candidate, ownership, and outstanding checks without copying a conversation transcript.

In this workflow, **ship means a verified commit pushed to a feature branch and an open pull request**. Merge and deployment are separate actions.

## Start in your coding host

Install the CLI from a reviewed checkout with Rust 1.95 or later:

```sh
cargo install --path . --locked
allstack -C /path/to/project init --skills-only --check
allstack -C /path/to/project init --skills-only
```

Then invoke a skill by name. Codex accepts `$allstack-plan`; Claude Code accepts `/allstack-plan`. Existing sessions may need reopening for skill discovery.

| Skill | Complete job |
| --- | --- |
| `allstack-shape` | Challenge demand, alternatives, and the outcome worth building. |
| `allstack-plan` | Resolve architecture, failure behavior, compatibility, and acceptance. |
| `allstack-review` | Find evidenced defects in an existing branch, diff, or PR. |
| `allstack-qa` | Exercise the actual UI, API, CLI, or job using available tools. |
| `allstack-ship` | Verify and publish the authorized change through managed delivery. |
| `allstack-recall` | Recover relevant project knowledge and inspect its sources. |
| `allstack-handoff` | Explicitly continue an active delivery in another host. |

Skills-only setup needs no Rust project, Chronis executable, task, or delivery run. Independent shaping, planning, review, and QA use the native host's tools and permissions. Shared original methods live in [skills/](skills); Rust packages them for both hosts with capability overlays. `allstack skills` inspects the bundled interface without opening stores.

See the [native-skills guide](docs/reference/native-skills.md) for installation, standalone memory, and enrollment into durable shipping. The [Spendcheck application](examples/spendcheck) demonstrates a concrete expense-report change through these methods.

## Ecosystem underneath

| Component | Owns |
| --- | --- |
| AllSource Core | Ordered workflow events and durable operation receipts |
| Chronis | Tasks, dependencies, claims, and task completion |
| Prime | Source-linked decisions, incidents, and relevant prior work |
| Allstack | Workflow transitions, handoff contracts, evidence binding, and reconciliation |
| Host adapters | Real `codex exec` and `claude --print` invocations with bounded process ownership |
| Git and forge | Source content, branches, commits, and pull requests |

Workflow methods and repository policies are versioned data. Generic mechanics belong in Rust. Optional browser, research, evaluation, and hosted-service integrations follow after the core workflow is proven.

## Durable delivery and recovery

For managed delivery, initialize a real project policy and Chronis:

```sh
allstack --help
allstack -C /path/to/target-repository init --check --profile rust
allstack -C /path/to/target-repository init --profile rust
allstack -C /path/to/target-repository doctor
allstack -C /path/to/target-repository catalog
allstack -C /path/to/target-repository catalog bugfix
```

`init` bundles workflow methods, indexed CLI documentation, repository-local Codex/Claude guidance, private-store ignores, and Chronis initialization. The explicit Rust profile requires `Cargo.toml` and `Cargo.lock`; an existing valid `--policy <path>` supports custom verification. Conflicting managed files are preserved and block writes. Reruns fill missing resources without replacing existing policies or repository instructions. Review initialized inputs before committing them.

The [installed-bootstrap verification report](docs/reports/2026-10-24-repository-bootstrap-verification.md) records fresh setup with actual Chronis and generated-policy execution while source-checkout reads were denied.

The [fresh-repository delivery report](docs/reports/2026-10-24-fresh-repository-delivery-verification.md) extends that proof through authenticated Codex → Claude Code → Codex handoffs, five executed acceptance tests, and a newly published [example PR](https://github.com/all-source-os/allstack-example/pull/1).

Managed delivery needs an authorized live feature branch, a GitHub `origin`, and an explicit verification policy. Use `start` for a clean managed journey or `adopt` to enroll completed native work with its existing accepted plan. `current` discovers the active job behind the skill interface. Both coding hosts retain normal authentication and permissions. `doctor` probes availability without authenticating them. The [CLI guide](docs/reference/cli.md) documents the underlying recovery protocol.

Select `delivery` for an accepted change or `bugfix` for evidence-led diagnosis and repair with regression coverage. Pass `start --workflow bugfix`; omitting the option retains `delivery`. Both share the same durable lifecycle and publication gates. See the [workflow catalog](docs/reference/workflow-catalog.md).

The [catalog verification report](docs/reports/2026-10-25-workflow-catalog-verification.md) records installed selection, legacy-history readback, and an authenticated Codex/Claude bug-fix journey with recovered host calls and frozen regression coverage.

Each command holds an exclusive kernel lock on the local store. There is no persistent coordinator daemon or IPC service. Epoch checks fence Allstack-managed transitions; native tools used outside Allstack remain outside that boundary.

Snapshots require explicit `git add -- <path>` for new files. They reject symlinks and recognized sensitive filenames, and allow at most 4 MiB per file, 64 MiB total, and 10,000 files. These checks do not detect every possible secret. Keep private stores and process evidence under ignored `.allstack/`.

## Documentation

- [Documentation index](docs/README.md)
- [Native skills](docs/reference/native-skills.md)
- [Cross-host workflow design](docs/plans/2026-10-24-cross-host-workflow-design.md)
- [Release acceptance scenarios](docs/reference/release-acceptance.md)
- [CLI and recovery guide](docs/reference/cli.md)
- [Delivery workflow data](workflows/delivery.toml)
- [Workflow catalog](docs/reference/workflow-catalog.md)
- [Codex host guidance](hosts/codex/SKILL.md)
- [Claude Code host guidance](hosts/claude-code/SKILL.md)

Acceptance checkboxes remain open until retained evidence establishes each claim. Source tests and fixture definitions alone cannot establish actual cross-host behavior.

## Relationship to gstack

[gstack](https://github.com/garrytan/gstack) demonstrates opinionated planning, review, QA, and delivery workflows. Allstack follows that specialist interaction model with original methods. AllSource, Chronis, and Prime provide durable history, ownership, and source-linked continuation underneath the native skills. Any future reuse must retain the applicable upstream and third-party notices.

## License

Apache-2.0. See [LICENSE](LICENSE).
