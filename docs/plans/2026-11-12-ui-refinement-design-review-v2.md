---
design: docs/plans/2026-11-12-ui-refinement-design.md
design-sha256: 4dca3ccb759c3cf8964e2314e0cf8427066f6905be3726fd0763a3aabc930cf6
verdict: findings
---

1. **Blocking — Capture integrity:** Nginx expands server-side includes, so the response bytes cannot simply be compared with the unexpanded source file. Define the deterministic transformed response, redirects, encoding and missing-asset behavior.
2. **Blocking — Coverage and adapter:** Define the inventory and adapter schemas, changed-file classification, route discovery, failure behavior, implementation ownership and backwards compatibility. The existing prose leaves these choices to the implementer.
3. **Blocking — Pilot verification:** Specify the client and page-bundle build inputs, preview staging and serving steps, noindex assertions and negative fixtures. Funny-face printing and name-art input are separate printable routes.
4. **Should fix — Portability and state:** Identify a candidate second-stack product and keep its verification explicitly open. Treat the private Free plan as a dated observation rather than permanent state.

Independent review used the recorded digest and inspected relevant product source. These findings remain open for this revision.
