# UI refinement ritual: design

**Status:** draft. The [concepts brief](2026-11-12-ui-refinement-brief.md) records the review findings. This document is not approved and does not claim that its proposed checks are implemented.

## Decision

Keep one portable `ui-refinement` method and use the existing ritual for delivery. Each product owns an indexed page/state inventory, source-bound evidence, decisions and executable adapters. Existing product gates remain in place. Automated checks establish only their stated predicates; a human reviews the interface and its evidence. ReadyPlayDays is the first pilot. Its private Free-plan status was observed on 2026-11-11; recheck current settings before claiming enforcement.

## Current behavior

The [UI method](../../skills/allstack-design/SKILL.md) inventories affected pages, records decisions and requires real gate checks. The two skills repositories carry host-specific copies. The ReadyPlayDays pilot gate checks declared UI-source coverage, image signatures, source blobs, snippets and declared check outcomes. It cannot establish that a screenshot depicts the claimed build, that every affected page is listed, or that a named reviewer actually approved it. Its local hook blocks only clones where installed; contributor-wide merge enforcement is not established.

The 2026-11-11 pilot observed missing activity routes and broken printable controls. A local candidate exists in another checkout, but its combined staged build and deployment were not verified in that review. Do not treat that observation as the current deployed state without rechecking it.

## Design

### Coverage and decisions

Maintain a versioned, indexed inventory in the product repository. Each row identifies a page or route, entry path, state, user goal, source and shared dependencies, supported viewport/theme/input combinations, and `reviewed`, `blocked`, `excluded` or `unverified` status with rationale. Dynamic pages may share a template check, but every distinct user journey and every route selected for the contribution gets its own outcome. Report totals and the denominator; do not count blocked or excluded states as reviewed.

Compute the affected set from changed sources and the inventory's dependency map. If a shared token, component or unmapped source may affect more routes, expand the set or require a named reviewer to record why a smaller set is sufficient. Refresh the map when routes or components change. Human review examines task completion, understandable copy, hierarchy and visual consistency; an automated pass is not that review.

The proposed inventory v2 is a JSON object with `version: 2`, `product`, `discovery_revision`, `discovery_sources` and `rows`. Each row has a unique `id`, `route`, `entry`, `state`, `goal`, `sources`, `viewports`, `themes`, `inputs`, `status`, `rationale` and `evidence_refs`. Status is exactly `reviewed`, `blocked`, `excluded` or `unverified`; all but `reviewed` require a rationale. Discovery compares router declarations, generated route manifests, navigation links and adapter-reported runtime routes. Dynamic or authenticated routes that cannot be enumerated are recorded as unverified. A new, deleted or unmapped UI source prevents a complete-coverage result until classified. Coverage reports each status and the total; their sum must equal the discovered denominator. The inventory and dependency map are reviewed product data, not a claim that static analysis discovers every possible state.

### Evidence and adapters

A capture record identifies the source revision, dirty-diff digest when applicable, build inputs and generated-asset digests, served origin and response identity, route/state, viewport, capture timestamp and image digest. Compare before and after only when their environment and state match, or explain the difference. A changed dependency invalidates affected captures unless the reviewer records an evidence-based exception. Redact sensitive data; apply product retention and access rules before storing screenshots.

The product adapter exposes bounded `build`, `start`, `capture` and `check` operations, selected by a versioned product-owned configuration rather than an agent-provided shell string. Each returns an explicit success, failure or unsupported result and identifies its inputs, process owner and deadline. The workflow records actual commands and output; it does not invent an adapter when one is unavailable. `autoresearch-rs` is optional. When used, freeze its manifest, evaluator, objective and candidate scope before the baseline, and preserve the actual keep/discard evidence.

Adapter configuration v1 identifies the product, protocol version, executable digest, working directory, allowed operations, arguments and allowed network destinations. Each request identifies its operation, immutable input digests and deadline; each response returns `success`, `failed` or `unsupported`, observed inputs, exit status and output digests. The runner resolves the configuration from the checked-out product and rejects unknown versions, operations, executables and destinations. The owning process terminates and reaps children at the deadline. A start operation returns a runner-owned handle that is stopped and reaped after capture.

The capture runner starts the declared build in an isolated environment, records the process and configuration identity, and hashes the built output before serving it. It records the requested URL, every redirect and final URL, response status and relevant headers, and hashes decoded HTML and locally served assets observed during capture. Request identity encoding for reproducibility; reject unexpected redirects, failed local assets and unapproved external assets. For server-side includes, separately record the raw staged files and their expanded HTTP response: the expanded response is checked against a deterministic reference served from the same staged bundle and preview configuration, not against raw HTML bytes. A verifier compares observations with the manifest, capture parameters and screenshot digest. This demonstrates a bounded chain from the declared local build to observed responses; it does not prove the screenshot is truthful against a compromised runner, certify accessibility, or identify a separate deployment. Recheck the deployed response independently when deployment is authorized.

Retain existing ReadyPlayDays review records as historical evidence. The current `docs/ui-reviews/` JSON gate remains in place. A v1 reader preserves each old record unchanged and maps absent v2 fields to `unverified`; it never manufactures provenance. New v2 records carry the version and full capture bindings. Until the adapter and migration are implemented, missing bindings remain unverified and no new enforcement is claimed.

### Gates and rollout

Keep source-bound evidence checks, noindex, CSP and all existing test gates. For ReadyPlayDays, run `tooling/xtask/src/ready_play_preview.rs` through its actual staging entrypoint and serve the output with `deploy/nginx.preview.conf`. Check `/`, `/?activity=paper-shape-trail`, `/activities/paper-shape-trail`, `/guides/rainy-day-activities`, the staged printable linked from `/?activity=color-funny-faces`, and a missing activity route. HTML pages must carry their applicable noindex metadata; responses, including 404s, must carry `X-Robots-Tag: noindex, nofollow, noarchive`. The missing route must return 404 with a usable recovery link. The printable must open print preview under the existing CSP, and name input must update its preview. Never expose or print real user data during the test.

Build the web client with `dx build --package web --platform web --fullstack false`, generate the page bundle with explicit `READY_PLAY_SEO_DIR` and `READY_PLAY_SEO_ORIGIN`, then set `READY_PLAY_PREVIEW_BASE` for `cargo xtask stage-ready-play-preview`. Serve only the resulting local staging directory through the checked-in preview Nginx configuration. Record exact values and output digests. Check noindex headers on each response, including redirects and the missing route, and inspect applicable robots metadata in served HTML or the rendered document. Negative fixtures remove each required header or metadata and must fail. Follow the name-art link separately from the funny-face printable: enter a name and verify its preview changes; verify print preview on the funny-face printable under the existing CSP. Missing assets, external redirects and failed page generation fail the pilot rather than being counted as reviewed.

Test that the existing gate rejects a missing record and accepts a valid one. Distinguish an installed local hook, CI visibility and a forge-enforced required check; report who can bypass each. Do not claim a universal merge block unless a real rejected merge proves it. Where the current private Free plan offers no such policy, report the limitation and preserve local and CI checks. The plan and visibility were last checked on 2026-11-11 and require a fresh check before reporting current enforcement.

Hook installation must preserve the previous hook chain, document its clone-wide effect and recovery, and account for unsupported platforms. No change to repository visibility, plan, deployment or contributor permissions is authorized by this design.

### Files and ownership

- `skills/allstack-design/SKILL.md` and `skills/allstack-ui-handoff/SKILL.md`: portable workflow and reproducible handoffs.
- `decebal-claude-skills/skills/ui-refinement/` and `decebal-codex-skills/skills/ui-refinement/`: host packaging and attribution; keep behavior aligned.
- ReadyPlayDays `docs/ui-reviews/`, `docs/evidence/`, its existing UI gate and preview staging: product-owned inventory, evidence, noindex and regression checks. Coordinate with the active product owner before editing shared files.
- This indexed design and its independent review files: decision, evidence limits and approval input. Existing PRs remain the delivery surfaces for their repositories.

## Alternatives

Separate UI tracker and approval flow duplicates the ritual's state. A screenshot-only gate cannot judge whether a user can complete a task. A local hook alone cannot guarantee enforcement across contributors. These alternatives do not meet the stated requirements.

## Semantic ISA

This is a proposed instruction contract for the workflow, not an installed security kernel. Existing host permissions, repository controls and external service policies remain authoritative. The workflow cannot prevent an actor with separate direct access from bypassing it.

### Side effects and instruction registry

| Instruction | Core / property | Data and binding | Sink and verification |
| --- | --- | --- | --- |
| `review.propose` | Cognitive / untrusted proposal | Inventory references and user goal; returns proposed coverage and findings | Response or model disclosure; verify permitted context and destination. |
| `evidence.read` | Memory / authorized I/O | Repository and evidence references; returns source-bound observations | Read/disclosure; verify access and confidentiality before loading or sharing. |
| `VERIFY(evidence)` | Normative / deterministic check | Revision, input and image digests, route/state and policy snapshot; returns a scoped receipt | No external write; reject missing, stale or mismatched inputs. A digest does not prove usability. |
| `record.write` | Execution / effectful write | Product path, expected revision and evidence digest; returns local write receipt | File write; verify workspace scope, current revision and ownership. |
| `check.run` | Execution / supervised process | Adapter command, input identity and deadline; returns exit status and artifacts | Process and possible network access; verify declared scope and destination, own and reap the process. |
| `contribution.publish` | Execution / effectful write | Exact repository, ref, expected remote revision and commit; returns verified destination state | Remote write; verify explicit authority, non-trunk destination, current branch and required checks; reconcile uncertain outcomes. |
| `progress.assess` | Meta-cognitive / advisory | Inventory and receipts; returns coverage status | Response/disclosure; cannot change approval or authority. |

### Typed bindings and taint

All bindings are closed, versioned objects. Identifiers, paths and URLs are UTF-8 strings of 1–2048 characters; SHA-256 digests are 64 lowercase hexadecimal characters. Arrays contain at most 10,000 entries; timestamps are UTC instants. Unknown fields, unsupported versions and invalid paths are refused. The host resolves principal, approval and current policy; model-supplied actor or approval fields are not accepted.

| Instruction | Required input fields, in addition to `version` | Closed output fields |
| --- | --- | --- |
| `review.propose` | `inventory_digest`, `goal` | `status`, `proposals`, `error` |
| `evidence.read` | `repository`, `revision`, `paths`, `destination` | `status`, `source_digests`, `error` |
| `VERIFY(evidence)` | `repository`, `revision`, `build_digest`, `route`, `state`, `viewport`, `image_digest`, `policy_revision` | `status`, `receipt`, `error` |
| `record.write` | `operation_id`, `repository`, `path`, `expected_revision`, `record_digest`, `content_ref`, `evidence_receipts` | `status`, `receipt_id`, `observed_revision`, `error` |
| `check.run` | `operation_id`, `adapter_id`, `adapter_version`, `operation`, `input_digest`, `deadline_seconds` | `status`, `exit_code`, `artifact_digests`, `error` |
| `contribution.publish` | `operation_id`, `repository`, `ref`, `expected_remote`, `commit`, `check_receipts` | `status`, `receipt_id`, `observed_revision`, `error` |
| `progress.assess` | `inventory_digest`, `receipt_ids` | `status`, `coverage`, `error` |

`operation` is one of `build`, `start`, `capture` or `check`; deadlines are positive integers capped at 300 seconds. All outputs use `status: success | refused | failed | uncertain`; unavailable values are null and `error` is a bounded reason code. `content_ref` names immutable workspace content whose digest must match `record_digest`. The schema is a design contract to implement and test, not an existing executable policy.

Screenshots, repository content, web content, model outputs and delegated reports are untrusted inputs. Track lineage, confidentiality, integrity/review state and authority as separate labels through capture, storage, summaries and delegation. `VERIFY(evidence)` is the proposed deterministic predicate `evidence_matches_v1`: the receipt issuer validates all specified digests, route/state/viewport, principal scope, product and policy revisions, capture deadline and expiry against a captured registry snapshot. A receipt is valid only for those inputs. The predicate cannot grant permission or upgrade a human judgment. The existing evidence gate does not yet implement this complete contract.

Additional reachable effects must be inventoried at the actual host boundary: browser navigation and remote reads can disclose URLs; captures and reports write files; builds and server startup spawn processes; tracker edits mutate task data; hook installation changes clone configuration; delegation discloses scoped context; PR creation and updates write to the forge. Each requires the current host's permission and a named executor-side verification. Email, billing, repository visibility, deployment and contributor administration are outside this workflow's requested authority. A skill or this design cannot fence direct actions by other tools or users.

Before a write, re-resolve the canonical path, check ownership, revision and current authority, and commit atomically where the adapter supports it. Use `operation_id` to detect duplicate same-payload requests and reject conflicting reuse. Recheck volatile access at the boundary. An uncertain remote result requires destination readback before retry. These are required properties, not claims about every existing host.

### Refusal and test matrix

| Sink | Refusal fixture | Allowed fixture and verification |
| --- | --- | --- |
| Read or disclosure | Missing access or sensitive data sent to an unapproved destination | Authorized source and destination; retain provenance and redact as required. |
| Evidence record write | Stale source, changed image or path outside the owned workspace | Matching revision and digests; write only the authorized record. |
| Adapter execution | Unknown command, unbounded process or undeclared network destination | Declared command and inputs; timeout, exit status and owned process are recorded. |
| Remote contribution | Trunk ref, stale lease, absent authority or failed gate | Authorized branch, matching lease and passing required checks; read back the remote ref. |

Test each refusal and success at the real boundary. Add fixtures for missing/stale screenshots, wrong build and route, a shared-source change affecting multiple routes, unmapped/new/deleted sources, exclusions and partial denominators; each must reject or explicitly report unverified. Test receipt reuse against a different resource, source-label round trips, duplicate and conflicting operation IDs, concurrent edits, revoked authority, interrupted capture and uncertain remote response. A timeout or uncertain remote response triggers readback before retry. Unknown policy, unsupported platform and unvisited page remain unverified, not passed. Reviewers compare every new sink in the final implementation to this table and name the corresponding check and test.

## Acceptance and confidence

- A reviewer can follow one ReadyPlayDays journey from page inventory to matching before/after evidence, decision, code example and actual gate result.
- A missing or stale record fails the applicable implemented check; a valid record passes. The record states whether the check blocks a local push, is visible in CI, or is required for merge.
- Existing noindex and validation checks pass on the source-bound staged build and served responses; missing routes and printable interaction are checked without weakening CSP.
- A second product with a different stack exercises the adapter contract before portability is declared proven.
- Independent reviewers test the design against current code and evidence. Human approval applies only to the exact reviewed design digest.

**Confidence: sand.** The method and some local checks exist, but full affected-state coverage, source-to-served-build binding, universal enforcement and a second-stack run are not established. Verified staged journeys, refusal tests and a second adapter would move this toward rock.
