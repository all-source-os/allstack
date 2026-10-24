# Allstack

Plan, build, verify, and ship across coding agents, with durable history and evidence tied to the delivered change.

**Status: design scaffold.** This repository specifies the first complete workflow. It does not yet contain an installable CLI, a working coordinator, or validated host integrations. Command examples describe the proposed interface.

Allstack combines [AllSource](https://github.com/all-source-os/all-source) for run history, Chronis for task state, and Prime for recall. Coding agents remain responsible for model execution. The first release targets Codex CLI and Claude Code on one machine, with an explicit handoff between them.

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
| Host adapters | Capability detection and host-specific skill/tool integration |
| Git and forge | Source content, branches, commits, and pull requests |

Workflow methods and repository policies are versioned data. Generic mechanics belong in Rust. Optional browser, research, evaluation, and hosted-service integrations follow after the core workflow is proven.

## Read the design

- [Documentation index](docs/README.md)
- [Cross-host workflow design](docs/plans/2026-10-24-cross-host-workflow-design.md)
- [Release acceptance scenarios](docs/reference/release-acceptance.md)
- [Proposed workflow data](workflows/delivery.toml)

The design distinguishes existing ecosystem primitives from integrations that Allstack still needs to implement. Acceptance checkboxes remain open until real host journeys and failure recovery have been exercised.

## Relationship to gstack

[gstack](https://github.com/garrytan/gstack) demonstrates opinionated planning, review, QA, and delivery workflows. Allstack uses that workflow shape as a reference and focuses its first release on durable cross-host continuation. No gstack source or skill text is included in this scaffold. Any future reuse must retain the applicable upstream and third-party notices.

## License

Apache-2.0. See [LICENSE](LICENSE).
