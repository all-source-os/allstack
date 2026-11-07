---
name: allstack-reflect
description: Write a retrospective from tracker history, run evidence, and recalled decisions, and flag approved designs whose code has moved since approval.
---

# Reflect on delivery

Read evidence, then write one report. Do not edit product code, close tasks, merge PRs, or change approvals. A reflection that cannot read a source states that gap instead of inferring the result.

## Gather evidence

Run each read one per call:

- Tracker history: `cn list --all --toon`. The output can be large; save it to the scratchpad and search it rather than reading it whole.
- Managed runs: `allstack current` for an active run, and `allstack evidence <run>` for gate receipts of a shipped or completed run.
- Prior context: `allstack recall <query>`. Open each cited source before relying on it.

## Drift check

For each design under `docs/plans/*-design.md`:

1. Run `allstack spec check <path>`. Record the gate reason.
2. Find the design's last commit with `git log -1 --format=%H -- <path>`.
3. List commits since that point that touch the paths the design names, with `git log --oneline <commit>..HEAD -- <path>...`.
4. Flag a design whose named code moved after its approval. Give the commits and the paths. A design with no approval is reported as unapproved, not as drift.

## Write the report

Write `docs/reports/YYYY-MM-DD-<slug>.md` and add it to `docs/README.md` in the same change. Include:

- Facts: runs, PRs, and task counts with their sources.
- What worked and what failed, each with its evidence.
- Drift flags, each with its commits.
- Gaps: any source that was unreadable or not checked.
- Decisions to keep, as a list the next session can act on.

Use plain engineering prose. Give numbers, paths, and commands rather than adjectives. Do not assign blame to a person.

Retain the accepted decisions with `allstack remember --kind decision --source <report> --summary "<decision>"`. Retain review results with `--kind review`. Memory is context only; it does not authorize a change.

## Report

Return the report path, the drift flags, and the list of unreadable sources.
