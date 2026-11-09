# Agentic harness glossary

This glossary maps the terms used in the harness proposal to the concepts that exist in Allstack today, and says where a term has no implementation yet. The lifecycle runs Think, Spec, Plan, Build, Review, Ship, Reflect.

| Term | Meaning here | Allstack concept | Status |
| --- | --- | --- | --- |
| Harness | The set of skills, hooks, and CLI commands that carry one change through the lifecycle | `allstack init --skills-only` plus the managed delivery commands | Implemented for skills and delivery. Guard hooks live outside this repository. |
| Skill / role | One stage or specialist, invoked by name in Codex or Claude Code | `skills/allstack-*/SKILL.md`, registered in `METHODS` in `src/infrastructure/skills.rs` | Implemented for 11 skills |
| Rule / guard | A deterministic check that denies or allows a tool call, independent of the model | Repository gates in `workflows/*.toml`; host hooks are configured outside Allstack | Gates implemented. Host hook installation is not part of Allstack. |
| Elephant | The drafting session that writes a design and resolves review findings | `allstack-spec` skill, which writes `docs/plans/YYYY-MM-DD-<slug>-design.md` | Implemented as a skill |
| Goldfish | A fresh-context reviewer that sees only the draft path and the repository root | `allstack spec review <path> --host codex\|claude`, which starts a read-only host process | Implemented. The reviewer host runs in a new process with no caller conversation. The upstream EGM plugin adds a deeper review that a person starts; it does not replace `spec review` on the final bytes. |
| PRD | A product requirements document for a planned change | `docs/plans/` design documents, plus the external `claude-prd` skill | Partly implemented. Allstack has no PRD template. |
| Bead / epic | A tracker task and its parent grouping | Chronis (`cn`) tasks and epics, read through `cn ready --toon` | Delivery claims a single task per run. Epic-level bridging is not implemented. |
| Ledger | The append-only record of decisions, events, and receipts | AllSource run history under `.allstack/events`, approval facts in the same store | Implemented for run history and spec approvals |
| Kernel / ISA | A deterministic layer that admits or refuses typed operations | Not implemented. The `semantic-isa` skill describes the design; no kernel exists in this repository. | Water. Requires a spike on one agent. |
| Verdict | The reviewer's explicit outcome: `clean`, `findings`, or `unknown` | `VERDICT:` line parsed by `application::spec::verdict` | Implemented. A missing verdict is `unknown`, which is never treated as clean. |
| Evidence report | A retained record of gate outcomes and their digests | `allstack evidence <run>` for gate receipts; reports under `docs/reports/` | Implemented for managed runs |
| Confidence (rock / sand / water) | How well the work is understood, not how large it is | Used in design documents and reports to classify open risks | Documentation convention. Rock means bet it, sand means shape it first, water means spike it first. |

## Approval binding

A design is approved for one exact byte sequence. `allstack spec approve` records the SHA-256 digest of the file, the approver name, and the review receipt that covered that digest. `allstack spec check` exits 0 only while the latest approval's digest equals the file's current digest and the cited receipt still exists and names the same digest. Any edit changes the digest, so the approval stops matching and the design must be reviewed again.

`--without-review` records an approval with no receipt. It exists for an explicit override and is visible in the approval record.

## Stage ownership

- Think: `allstack-shape`
- Spec: `allstack-spec`, with `allstack spec review`, `approve`, and `check`
- Plan: `allstack-plan`. The external `claude-beads` skill is expected to run `allstack spec check` before it cuts beads. Allstack does not enforce that call.
- Build and Ship: `allstack-ship` through managed delivery
- Review: `allstack-review` and `allstack-qa`
- Reflect: `allstack-reflect`
- Routing: `allstack-harness` names the current stage and loads the next skill
- Diagnosis: `allstack-investigate`, which may skip Think and Spec for a reproduced defect
