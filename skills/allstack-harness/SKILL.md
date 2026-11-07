---
name: allstack-harness
description: Name the current lifecycle stage of a change (Think, Spec, Plan, Build, Review, Ship, Reflect) from read-only repository state and load the next specialist skill.
---

# Route the lifecycle stage

Route only. This skill names the stage, records skipped stages, and loads the next skill by name. It does not design, review, implement, commit, or publish. Read repository policy before acting.

## Inspect state read-only

Collect these facts before naming a stage. Run them one per call. Every command reads repository state except `allstack spec check`, which takes the run coordinator lock.

- Branch and liveness: `git branch --show-current`, then `gh pr list --head <branch> --state all --json number,state,title`. A `MERGED` branch is dead and starts a new branch.
- Active managed run: `allstack current`. A returned run names its stage; use `allstack status <run>` for detail.
- Tracker: `cn ready --toon` for unblocked beads. Use `cn list --all --toon` only when the answer depends on completed work.
- Design documents: list `docs/plans/*-design.md` for the change. For each one that matters, run `allstack spec check <path>`. Exit 0 means the current digest is approved. Exit 1 gives the reason: `unapproved`, `changed since approval`, or `unreviewed`.
- Shape and plan documents: check the indexed `docs/plans/` entries named by the request.

Absence of state is a fact to record, not a reason to guess. A tool that cannot run leaves its stage unverified.

## Name the stage

| Stage | Named when | Next skill |
| --- | --- | --- |
| Think | No demand, scope, or alternatives are written for the request | `allstack-shape` |
| Spec | A design is needed and no approved design matches the current digest | `allstack-spec` |
| Plan | An approved design exists and no beads or accepted plan cover it | `allstack-plan`, then `claude-prd` and `claude-beads` |
| Build | Beads are ready on a live branch and implementation is not yet verified | `allstack-ship` (managed delivery through `start`, `plan`, `build`, `verify`) |
| Review | Implementation exists on the branch and has no review receipt | `allstack-review`, then `allstack-qa` |
| Ship | Verification passed and no open PR exists for the branch | `allstack-ship` |
| Reflect | The run shipped or the PR merged | `allstack-reflect` |

The Plan stage must not cut beads for a design whose `allstack spec check` does not exit 0. `claude-beads` is expected to run that check first; this repository does not enforce it. If the check fails, report the reason instead of working around it.

## Bug fixes skip Spec

A defect with a reproduction is a bug fix. Record `Think` and `Spec` as skipped with the reason, then route to `allstack-investigate` for diagnosis and to the `bugfix` catalog workflow for durable delivery. A defect that changes intended behavior is a feature change and returns to Spec.

## Report

Return one short block: the branch and liveness fact, the named stage, each skipped stage with its reason, the single next skill, and any state that could not be read. Load the next skill only after this block is written. Do not load a later stage in the same turn.
