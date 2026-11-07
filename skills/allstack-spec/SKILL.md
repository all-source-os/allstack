---
name: allstack-spec
description: Draft a design document, obtain a fresh-context host review, resolve findings, and record a human approval bound to the design's sha256 digest.
---

# Specify the design

The drafting session is the author. A review by `allstack spec review` runs in a separate host process that sees only the draft path and the repository root. Approval belongs to a human. Do not approve on your own judgment.

## Draft

Write `docs/plans/YYYY-MM-DD-<slug>-design.md` in this repository's docs layout. Do not create a `docs/designs/` directory. Add the document to `docs/README.md` in the same change.

Cover the complete job a person completes when the change works, its trigger, inputs, outputs, and failure behavior. State non-goals. Name the observable acceptance checks. Mark each open risk with a confidence tier: rock (bet it), sand (shape it first), or water (spike it first). Do not write time estimates.

When the design is an agent or tool-using system, frame it with semantic-isa: classify each environment-altering operation into a typed instruction, state the governance property the kernel enforces, name the typed bindings, and trace taint from untrusted inputs to every sink. Verification clears taint only through a named check.

## Obtain a fresh-context review

Run the review from the repository root:

```sh
allstack -C <repo> spec review docs/plans/<file>-design.md --host codex
```

Use `--host claude` for Claude Code. The command prints JSON with `receipt`, `digest`, `verdict`, and `findings`. Read the findings from the receipt path. A nonzero exit means the host failed; report it and do not treat the design as reviewed.

## Resolve and loop

Resolve each finding by changing the draft, or by stating in the draft why the finding does not apply, with the evidence. Any edit changes the digest, so run `spec review` again on the new bytes. Repeat until the verdict is `clean`. A `findings` or `unknown` verdict is not clean.

## Ask the human

When the verdict is clean, show the human the design path, its digest, the review receipt path, and any open findings you rejected with reasons. Ask for an explicit yes. Only on that yes run:

```sh
allstack -C <repo> spec approve docs/plans/<file>-design.md --by "<approver name>"
allstack -C <repo> spec check docs/plans/<file>-design.md
```

`check` must exit 0 before the Plan stage starts. Pass `--without-review` only when the human asks for that override in those words. Record the override in the report.

Any later edit invalidates the approval. `spec check` then reports `changed since approval`; return to review.

## Report

Return the design path, its digest, the review verdict and receipt path, the approval result or the explicit reason none was recorded, and the rejected findings with their evidence.
