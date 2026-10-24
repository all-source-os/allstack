---
name: allstack-delivery
description: Continue a managed Allstack plan, build, verify, and ship workflow through Claude Code and Codex CLI with retained run state and explicit host handoff.
---

# Allstack delivery through Claude Code

Use the installed `allstack` CLI as the managed workflow boundary. Read repository instructions and the [CLI guide](../../docs/reference/cli.md) before starting. Install this skill through the host's ordinary skill mechanism; the adapter does not install skills automatically.

## Establish context

Run `allstack -C <original-repository> status <run>` and `resume <run> --epoch <current-epoch>`. Read the returned request, indexed plan, stage, active worktree, pending operation, and contextual Prime recall. Open recalled sources before treating them as requirements. No previous Codex conversation is needed.

Use the original repository as the CLI's `-C` target. Edit only the active detached worktree returned in run state. The coordinator has already claimed the run's Chronis task and read back the expected claimant; do not claim, release, complete, or replace it independently.

## Execute the accepted work

`plan <run> --epoch <epoch>` invokes real `claude --print` with planning permissions. `build <run> --epoch <epoch>` invokes it with `acceptEdits` and the adapter's explicit tool allowlist. Host authentication and ordinary permission rules remain active. A denied operation must remain visible; never add bypass flags.

Follow the frozen request and indexed plan. Explicitly stage each intended new file with `git add -- <path>` before checkpoint capture. Do not commit, push, create branches, alter run state, or modify the verification policy from a managed build. The coordinator owns final verification and publication.

## Transfer or recover

Use `handoff <run> --epoch <epoch> --to codex` for a sequential transfer. Read the returned epoch and active worktree. Do not continue editing the outgoing workspace. Stale epochs fence Allstack-managed transitions; native host tools are outside that boundary.

After failure, inspect `status`, `history`, active workspace, and private process evidence. Use `recover` for pending host or handoff work only after inspecting its effects. Recovery preserves current edits without replaying an uncertain host invocation. Use the original destination-specific `verify` or `ship` command to reconcile their pending effects.

## Verify and publish

Use `verify <run> --epoch <epoch> --message <conventional-commit-message>` only under recorded commit authority. Verification prepares the original feature checkout and runs frozen gates on the exact SHA. Positive Cargo/nextest execution counts are required; a planning summary or compilation alone cannot satisfy execution proof.

Use `ship <run> --epoch <epoch> --title <conventional-pr-title>` only under recorded publication authority. Completion requires actual remote/PR readback and reconciled Chronis/Prime outcomes. Merge and deployment remain separate. Keep private evidence, local stores, credentials, and source snapshots out of public documents.
