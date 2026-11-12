---
design: docs/plans/2026-11-12-ui-refinement-design.md
design-sha256: bb707e17837bdbb5c1644679e3eb2e35a0e2814833b7307d8a0a3cacd87ffe90
verdict: findings
---

1. **Blocking — Bindings:** The proposed `record.write` omits the record payload and operation identity. Most listed instructions lack closed input/output bindings, bounded values, authority and error contracts. `check.run` does not bind an allowed adapter operation.
2. **Blocking — Sinks and provenance:** The inventory omits capture, build/start, browser navigation, tracker, delegation, hook configuration and PR operations. Define what the proposed contract governs, where data is disclosed, and what remains outside its enforcement boundary.
3. **Blocking — Verification:** Required predicates, evidence issuer, principal, destination, revision, expiry and invalidation are not specified. A digest is identity evidence, not proof of origin, authority or visual quality.
4. **Blocking — Recovery and tests:** The draft lacks a commit-time authority check, stable operation identities, replay/conflict handling, uncertain-write reconciliation and per-sink refusal/success tests at the actual executor boundary.

Independent reviewer read the draft, Semantic ISA method and relevant source without the author's chat. Findings have not been resolved at this digest.
