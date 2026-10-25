# Allstack realignment: specialist workflows over durable ecosystem tools

Date: 2026-10-25. Status: implementation preview on PR #5. Native specialist methods are the primary interface; see the [application verification report](../reports/2026-10-25-native-skills-application-verification.md) for installed and authenticated evidence.

## Product promise

Allstack gives a builder opinionated product and engineering skills inside their existing coding host, with project memory, task ownership, and portable continuation supplied by AllSource, Chronis, and Prime.

A complete human job is: shape an application change, challenge its design, implement it, review its failure risks, exercise the real user flow, and publish a verified PR. A builder can invoke any specialist independently. If they change coding hosts, the next host can continue from source-linked decisions and evidence without copying a transcript.

## Evidence of drift

The current Allstack README leads with durable cross-host execution. Its normal journey exposes run IDs, epochs, clean indexed inputs, a frozen verification policy, managed worktrees, and explicit handoffs before a user reaches the intended engineering work. Those mechanics serve recovery and publication, but currently define the product interface.

The four current method files contain 5, 7, 5, and 7 lines respectively. They specify planning/building duties and coordinator restrictions. They do not yet supply the substantive product challenge, architecture tradeoffs, review heuristics, or application QA methods that distinguish a specialist toolkit. Line counts are a symptom; the missing decision procedures and independent entry points are the gap.

Upstream gstack presents specialist skills that a builder invokes directly for product thinking, planning, review, QA, and delivery. Its architecture combines opinionated skill text with concrete browser tools. Its current installation documentation also covers multiple coding hosts and outside-model review. Supporting Codex and Claude alone therefore does not establish Allstack's differentiation.

Sources inspected on 2026-10-25:

- [gstack README](https://github.com/garrytan/gstack#readme): specialist entry points, installation, and supported hosts.
- [gstack architecture](https://github.com/garrytan/gstack/blob/main/ARCHITECTURE.md): skill templates, shared generated contracts, and browser execution.
- [gstack builder ethos](https://github.com/garrytan/gstack/blob/main/ETHOS.md): user-directed decisions, existing solutions as inputs, and proof through executed behavior.
- [Current Allstack README](../../README.md), [CLI](../reference/cli.md), and [method files](../../workflows/methods/): current product surface and implementation boundary.

## What to retain and change

| Concern | Proposed direction |
| --- | --- |
| Specialist methodology | Make it the primary product: substantive, original methods with explicit inputs, judgment, outputs, and stop conditions. |
| Native host invocation | Install directly invocable Codex and Claude skills from shared method sources. Keep capability differences visible. |
| AllSource | Retain authoritative managed-run events and operation receipts; add skill-boundary records only where they support continuation or a user decision. |
| Chronis | Retain tasks, dependencies, and claims for work the user commits to doing. A standalone review must not require creating a delivery task. |
| Prime | Recall relevant decisions and incidents with source inspection; capture accepted decisions and useful outcomes with provenance. Keep execution state and permission outside memory. |
| Managed coordinator | Retain exact-commit verification, reconciliation, process ownership, and handoff safety. Offer it for complete delivery and explicit continuation rather than requiring it for every specialist invocation. |
| Catalog | Organize by complete human jobs and useful specialists. A new ID pointing at two generic prompts is insufficient acceptance. |
| Browser and ecosystem tools | Route through available host/browser/research tools. State actual capabilities; add integrations when a complete job needs them. |

## Approaches

1. **Native specialist skills backed by Allstack, recommended.** Keep the existing engine and add the missing methodology and invocation surface. Users work within their host; the engine supports durable delivery when selected.
2. **Maintain a gstack fork with ecosystem replacements.** This retains more upstream methodology but requires ongoing reconciliation of assumptions, generated content, tools, and host behavior. Choose it only if continuing upstream compatibility becomes a product requirement.
3. **Continue expanding coordinator workflow IDs.** This preserves the current trajectory but does not address the user's concern about intention and design.

Adopt gstack's emphasis on specialist judgment, direct invocation, and actual execution. Preserve Allstack's original implementation and method text. Any deliberate upstream text or code reuse must carry its applicable attribution and notices. Do not claim a clean-room rewrite is inherently better than adapting an existing method; choose against concrete fit and maintenance cost.

## Initial user-facing skills

These names define the installed specialist interface; real-host and application acceptance must be established by retained execution evidence:

| Skill | Complete job and output |
| --- | --- |
| `allstack-shape` | Challenge the requested outcome, users, evidence, constraints, and alternatives; produce an indexed decision-ready brief. |
| `allstack-plan` | Review architecture, failure cases, tests, and dependencies; produce an actionable implementation plan with explicit tradeoffs. |
| `allstack-review` | Inspect an existing branch or PR for concrete correctness and safety risks; report actionable source locations and supporting evidence. |
| `allstack-qa` | Exercise the application's actual UI, API, CLI, or job flow using available tools; report reproducible defects and retained execution evidence. |
| `allstack-ship` | Run applicable review and verification, then publish the exact authorized commit through existing managed publication. |
| `allstack-recall` / `allstack-handoff` | Recover relevant source-linked knowledge or explicitly transfer an active job to another supported host. |

Plan → build → verify → ship remains a supported end-to-end journey. Users may also ask for a review of an existing branch, investigate a defect, or plan a change without starting that journey. Skills must support these partial jobs honestly without inventing successful delivery state.

The first implementation should select the smallest complete application journey that exercises the intended specialist set. Add design, security, performance, or retrospective specialists when their own methods and acceptance scenarios are defined. Do not enumerate unsupported commands merely to resemble upstream breadth.

## Shared methodology and host adapters

Maintain original method sources with a compact capability and result contract. Render or package repository-local host skills from those sources, using Rust tooling where generation is necessary. Shared methodology includes the questions to answer, risks to inspect, required evidence, allowed actions, and expected result. Host overlays bind actual tools and invocation conventions; they must not duplicate the whole method.

Direct skill execution uses the native host's tools and current repository policy. It cannot pretend the current coordinator's plan/build permissions allow compilation or browser QA. Keep managed-run restrictions intact; introduce the required adapter capability explicitly when integrating a specialist into a managed journey. A prose instruction is not a permission grant or an execution fence.

Separate independently invocable specialist results from managed delivery transitions. A review finding can be retained with a source reference without fabricating a run stage, task completion, or successful publication. Existing AllSource run/event compatibility remains a requirement.

## Next end-to-end proof

Choose a concrete application change, with an observable before/after user flow, rather than using a catalog extraction as the primary demonstration. Reuse an authorized repository and publication destination; do not create another repository or branch without the required user instruction.

The proof must show a builder invoking the installed skills naturally in Codex and Claude, receiving substantive product and engineering feedback, implementing the application change, exercising its actual runtime, and publishing one verified PR. Include an intentional host change within that useful job. Inspect actual host calls, changed files, runtime behavior, and destination state before asserting success.

Acceptance also includes independent use: invoke review against an existing branch without a delivery run, and invoke planning without granting commit or publication authority. Recall should recover an accepted source-linked decision in a new host session. Durable continuation must retain the applicable work and evidence while keeping execution details behind the normal skill interface.

## Implementation and acceptance

The implementation packages the seven original methods for both native hosts, adds skills-only setup, retains standalone source-linked knowledge in Prime, and enrolls accepted native work into the existing delivery engine. AllSource and Chronis retain their existing authority boundaries. Application acceptance uses Spendcheck category filtering, independent native planning/review/recall, and exact-commit publication.

Use the already authorized `feat/refactor-workflow` branch and one PR for this design review and any accepted implementation. No branch rename or additional PR is implied by this proposal.
