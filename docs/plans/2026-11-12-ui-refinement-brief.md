# UI refinement ritual: concepts brief

**Prepared by:** Codex

**Date:** 2026-11-12
**Status:** proposed; not an approved design.

## Overview

A product team needs to review an interface page by page, retain the reasons for each decision, and prevent later changes from silently undoing those decisions. The method must work for websites and desktop applications without requiring a particular stack. ReadyPlayDays is the first pilot: its existing indexing restrictions, safety language, UI contracts and validation checks remain in force.

The [current proposal](2026-11-10-interface-review-ritual.md) describes the workflow. Its pilot record shows that route and layout checks, printable interactions, and selected user journeys have been exercised, but a complete page-and-state review and deployed verification have not been established.

## Analysis

Three fresh-context reviewers examined the proposal in grouped EGM lenses: design and alternatives; codebase and implementation readiness; and comprehension, structure and invention. Grouping reflects the available concurrent reviewer slots; it is not seven independent reviews. The following findings require resolution in the design and verification plan:

1. **Coverage:** Define a page-and-state inventory with a denominator and explicit reviewed, blocked, excluded or unverified outcomes. Shared-pattern sampling can reduce repeated visual checks, but does not establish coverage of an unvisited state. A change to a shared component or unknown dependency expands the affected-page scope until a reviewer justifies narrowing it.
2. **Evidence provenance:** Bind each capture to the source revision, any dirty changes, build and served assets, route, state, viewport and capture artifact. Define when a change invalidates evidence. The ReadyPlayDays evidence gate checks source identity and image signatures, but its own documentation says it cannot prove that an image depicts the claimed build or that a reviewer made the judgment.
3. **Enforcement:** Separate checks that can reject a contribution from human judgments of clarity and consistency. Record exactly where each gate runs and who can bypass it. A hook installed in one clone cannot block other clones or a merge. ReadyPlayDays remains private on its existing Free plan; contributor-wide merge enforcement has not been demonstrated under that constraint.
4. **Noindex and existing behavior:** Preserve the product's indexing restrictions and validation gates across generated pages, application routes, staged output, HTTP responses and rollback. The earlier local review did not establish all staged and deployed paths. Do not weaken CSP to restore printable controls.
5. **Pilot completion:** Match the generator, client and served bundle to the tested source. Recheck missing activity routes and printable behavior on the staged build. Cover keyboard, recovery and accessible layouts. The 840-route and 55-printable checks in the earlier report are bounded checks, not visual review of every state or proof of deployed fixes.
6. **Ownership and portability:** Define a minimal adapter contract for building, starting, capturing and checking a product, including unsupported and failed outcomes. Record hook migration, existing-hook preservation, recovery and platform limits. Test the method on a second stack before claiming portability has been proven.

Use one indexed inventory as the canonical page/state and evidence record. Link it from the existing ritual brief, design, plan, review and PR proof; avoid re-entering the same facts. Batch route and layout checks and reuse shared-component evidence only when the affected scope and build identity remain valid. Human review still examines every distinct decision-changing state. Retain only the screenshots needed to support decisions, with the product's privacy and retention rules; mark missing evidence unverified.

## Goals vs. non-goals

**In scope:** a repeatable, stack-independent review method; usable Claude and Codex skills; a ReadyPlayDays pilot with actual rendered journeys and regression evidence; durable design decisions and accountable contribution gates.

**Outside current authorization:** changing ReadyPlayDays repository visibility or paid plan, deploying product changes, declaring unvisited states reviewed, or treating a local hook as a contributor-wide merge restriction.

## Alternatives weighed

- **One workflow with a reusable method and product adapters:** proposed. Reuses the existing ritual's stages and artifact handoffs while leaving product-specific build, indexing and security checks in the product.
- **A separate UI delivery system:** not selected in the current proposal; it duplicates tracker, approval and review state without resolving evidence or enforcement gaps.
- **Automated screenshots as the design verdict:** not selected; screenshots and scores cannot establish whether a caregiver understands or completes the task.

## Key decisions and staging for next steps

- Carry the six findings into a reviewed design and acceptance matrix; do not mark the current proposal approved.
- Preserve the user's choice to keep ReadyPlayDays private on its Free plan. If no universal merge gate exists, state the enforcement limit and keep actual local and CI checks intact.
- Keep the ReadyPlayDays fixes under their existing owner's control. Verify combined source, generated assets, served responses and user journeys before calling the pilot complete.
- The next ritual stage requires a design with its Semantic ISA, two fresh-context design reviews and the ritual's approval gate. These have not yet been completed.
