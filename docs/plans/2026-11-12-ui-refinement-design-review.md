---
design: docs/plans/2026-11-12-ui-refinement-design.md
design-sha256: bb707e17837bdbb5c1644679e3eb2e35a0e2814833b7307d8a0a3cacd87ffe90
verdict: findings
---

1. **Blocking — Evidence provenance:** The draft requires build-to-capture binding but does not define how the actual served build, assets and captured state are observed. Self-declared digests cannot establish that relationship. Define the capture boundary and retain an explicit limit for what it cannot prove.
2. **Blocking — Coverage and compatibility:** The draft does not define affected-set closure, failure behavior for unknown mappings, migration of existing evidence records or rejection tests for shared and deleted sources.
3. **Blocking — Pilot acceptance:** The noindex and printable checks lack concrete staging paths, routes and expected response behavior. One journey alone cannot establish coverage of every affected state.
4. **Should fix — Portability:** The second-stack requirement names neither a candidate nor a way to report a missing runnable product without inventing a pass.

Independent reviewer read the draft and relevant code without the author's chat. Findings have not been resolved at this digest.
