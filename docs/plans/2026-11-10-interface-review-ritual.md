# Interface review ritual

Status: proposed. The `allstack-design` native method provides the operator workflow. The Ready Play Days experiment and a local contribution hook have been exercised; complete page coverage and reviewer enforcement remain open.

## Decision

Keep the method independent of application stack. Put each product's decisions, captures, examples and executable adapters in that product's indexed documentation and verification system. Allstack packages the method for each native host. Use source-linked memory only as an index to the actual record, consistent with Allstack's existing recall contract. The operator verifies the output and checks, as chief-of-staff-v2 does for delegated work.

For planned delivery, compose `allstack-design` with the separate [`ritual` or `ritual-full` flow](https://github.com/decebal/decebal-claude-skills/pull/96). The flow owns stage selection, its Chronis epic, design approval and handoffs. The UI method adds page inventory, design decisions and verification evidence to those artifacts. The full flow retains its preflight, Semantic ISA, independent reviewers and drift check. A standalone UI review does not require the flow.

Discover page families and shared components once. Batch deterministic checks over affected routes, retain per-page outcomes, and inspect each distinct user-facing state. Reuse the same captures and decision records in the plan, review and PR proof. Run scoped checks during iteration and the existing whole-product gates at the integration boundary. This reduces duplicate work without treating a batch count as visual review or a missing check as a pass.

## Record for each page

Record page identifier and entry point, user goal, relevant states, build identity, environment, widths or window sizes, baseline and final screenshots, observed findings, decision and rationale, alternatives, applicable shared patterns, source examples, exception and owner, checks and results. Include an inventory showing reviewed, blocked and excluded states. Maintain an index and supersede conflicting decisions explicitly. Evidence must identify the capture's actual build; an unrelated deployed preview cannot verify a local candidate.

## Gate contract

For each contribution affecting UI, identify all affected pages and states and run the project's actual checks. Required outcomes: the primary journey completes; labels and hierarchy are clear; shared patterns follow accepted decisions or document an exception; layouts reflow at supported sizes; keyboard, focus and applicable accessibility checks work; comparable screenshots identify the tested build; and reviewers resolve unexplained regressions. Missing evidence is unverified. Record exact commands or named reviewers and wire checks into an enforced local or merge gate before calling them blocking. A human judges visual hierarchy and clarity; metrics cannot certify them.

## Ready Play Days pilot

The available directory is a Rust v2 / Dioxus product. Its homepage and activity views are implemented in `apps/web/src/main.rs`; `App` selects product views and the activity finder provides a search field. The isolated pilot identifies the tested local source and retains screenshots and its design decision.

The adjacent `autoresearch-rs` product-web evaluator supplies loopback browser captures and bounded accessibility checks. A real pilot needs a Git-backed product checkout, a reproducible local build and server, frozen route/state/viewport fixtures, explicit hard gates and an objective that can discriminate a specific hypothesis. The runner requires committed frozen inputs and creates candidate commits and a run branch. Freeze those inputs before baseline; retain baseline, candidates, decision reasons and final verification. The evaluator's sampled accessibility output does not certify the whole keyboard journey or visual quality.

The Ready Play Days Git checkout holds the pilot record under `docs/reports/2026-11-10-interface-review.md`. A source-isolated `autoresearch-rs` run kept a client-build brand image fix and retained four viewport screenshot pairs and an experiment journal. Manual review covered the finder, an activity, journal, one printable and unavailable account and reviewer states. A subsequent contribution adds a source-bound page record and a Rust evidence gate. Its integration tests exercise accepted and rejected pushes and preservation of existing hooks. The local hook requires installation per clone; CI visibility does not establish merge enforcement. Native skills were installed in the pilot, but no independent host invocation has driven the complete workflow. Page-by-page coverage and human review remain open.
