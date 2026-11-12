---
design: docs/plans/2026-11-12-ui-refinement-design.md
design-sha256: 4dca3ccb759c3cf8964e2314e0cf8427066f6905be3726fd0763a3aabc930cf6
verdict: findings
---

1. **Blocking — Schemas:** Field-name tables do not define closed nested types, bounds, output variants and errors for all instructions.
2. **Blocking — Verification:** The proposed predicate and receipt omit some of the principal, destination, issuer, policy, time, expiry and observed-response inputs they claim to bind. Specify the pure decision and fail-closed dependency graph.
3. **Blocking — Side effects:** Browser navigation, capture, local staging replacement, tracker, hook configuration, delegation and forge operations are acknowledged but lack instruction bindings and sink-specific refusal tests.
4. **Blocking — Atomicity and recovery:** Define safe refusal where atomic preconditions cannot be enforced, stable operation identity, commit-time rechecks, bounded process termination and uncertain remote-result reconciliation.
5. **Blocking — Fixtures:** Add tests for failed verification, hostile source data, changed principal/payload/policy, receipt expiry, cross-resource reuse, replay, concurrency and actual executor boundaries. Unsupported must be distinct from pass.

Independent review used the recorded digest and inspected relevant source. These findings remain open for this revision.
