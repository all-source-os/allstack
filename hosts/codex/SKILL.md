---
name: allstack-delivery
description: Continue a managed Allstack plan, build, verify, and ship workflow through Codex CLI and Claude Code with retained run state and explicit host handoff.
---

# Allstack delivery through Codex

Use the installed `allstack` CLI as the managed workflow boundary. Read the repository's instructions and the [CLI guide](../../docs/reference/cli.md) before starting. Install this skill through the host's ordinary skill mechanism; the adapter does not install skills automatically.

## Establish context

Run `allstack -C <original-repository> status <run>` and `resume <run> --epoch <current-epoch>`. Read the returned request, indexed plan, stage, active worktree, pending operation, and contextual Prime recall. Open recalled sources before treating them as requirements. No previous conversation is assumed.

Use the original repository as the CLI's `-C` target. Edit only the active detached worktree returned in run state. The coordinator has already claimed the run's Chronis task and read back the expected claimant; do not claim, release, complete, or replace it independently.

## Execute the accepted work

`plan <run> --epoch <epoch>` invokes real `codex exec` in read-only planning mode. The coordinator records and indexes the returned plan. `build <run> --epoch <epoch>` invokes Codex with `workspace-write`. Host authentication and permissions remain those of the installed CLI. Never add permission-bypass flags to make a run pass.

Follow the frozen request and plan. Explicitly stage each intended new file with `git add -- <path>` before a checkpoint. Do not commit, push, create branches, alter run state, or modify the verification policy from a managed build. Required gates and publication belong to the coordinator.

## Transfer or recover

Use `handoff <run> --epoch <epoch> --to claude-code` for a sequential transfer. Read the returned epoch and worktree; the outgoing workspace becomes stale. A stale epoch cannot advance managed state, although native tools remain outside Allstack's enforcement.

If a managed command fails, inspect `status`, `history`, active workspace, and private process evidence. Resolve pending host or handoff operations with `recover` only after inspecting their effects. Recovery preserves edits and does not replay an uncertain host call. Use `verify` or `ship` for their destination-specific pending operations.

## Verify and publish

Use `verify <run> --epoch <epoch> --message <conventional-commit-message>` only under recorded user commit authority. It prepares the original feature checkout and executes frozen gates on the exact candidate SHA. Require actual positive Cargo/nextest execution counts; host testimony and source checks cannot prove the complete flow.

Use `ship <run> --epoch <epoch> --title <conventional-pr-title>` only under recorded publication authority. Read remote and PR receipts, Chronis completion, and Prime outcome status before reporting completion. Merge and deployment are separate actions. Keep raw transcripts, private stores, credentials, and snapshots out of public documents.
