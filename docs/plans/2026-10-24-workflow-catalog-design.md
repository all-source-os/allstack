# Selectable workflow catalog and evidence-led bug fixes

Date: 2026-10-24. Status: implementation preview on `feat/workflow-catalog`. See the [verification report](../reports/2026-10-25-workflow-catalog-verification.md) for installed resources, retained-run compatibility, and authenticated host evidence.

## Outcome

An installed Allstack executable lists available repository workflows, explains their methods and supported boundary, and starts either the existing `delivery` workflow or an evidence-led `bugfix` workflow. Both complete the same durable plan → build → verify → ship lifecycle across Codex CLI and Claude Code. Bug-fix planning establishes the reproduction and failed condition; implementation repairs that condition and supplies regression coverage.

PR #3 merged as `add44a61aa454f6c11f2e055914c4c333de7a4fa`. Its [fresh installed delivery proof](../reports/2026-10-24-fresh-repository-delivery-verification.md) establishes authenticated execution, exact-commit publication, Chronis completion, and embedded Prime readback. Catalog expansion builds on that behavior. It does not declare every item in the broader [release acceptance](../reference/release-acceptance.md) complete.

## Approaches considered

1. **Repository catalog with shared lifecycle, selected.** Versioned catalog entries select workflow definitions and original method documents. Rust validates the supported lifecycle and safety contract. This provides a distinct new job while retaining existing handoff, verification, publication, and recovery mechanics.
2. **More standalone host skills.** Easy to discover inside individual hosts, but selection and evidence would remain disconnected from the durable run. This does not satisfy complete cross-host continuation.
3. **An arbitrary workflow graph engine.** Could express review-only or deployment jobs, but requires new transitions, authority rules, recovery semantics, and terminal outcomes. Current acceptance needs no such engine.

The first expansion adds `bugfix`. Independent review and behavior-preserving refactoring need their own complete acceptance scenarios before becoming supported catalog entries.

## User flow

```sh
allstack init --check --profile rust
allstack init --profile rust
allstack catalog
allstack catalog bugfix
allstack start --workflow bugfix --request docs/requests/repair.md \
  --policy workflows/verification.toml --host codex --allow-commit --allow-publish
```

`catalog` lists repository-defined entries; supplying an ID returns its description, definition path, methods, supported hosts, lifecycle, and digests. Discovery validates files without opening mutable AllSource or Prime stores or creating Chronis tasks. An absent catalog in a previously initialized repository exposes legacy `delivery` only. Unknown IDs fail with available choices.

Omitting `--workflow` preserves `delivery`. Explicit selection freezes the selected definition and method bytes before model execution. Normal `plan`, `handoff`, `resume`, `build`, `verify`, and `ship` commands continue using the returned run and epoch. Catalog entries cannot grant commit or publication authority.

## Data and engine boundary

- `workflows/catalog.toml` holds the versioned entry list, with unique IDs and contained definition paths. Verification policies are not discovered as workflows by scanning every TOML file.
- `workflows/bugfix.toml` declares the same supported lifecycle, hosts, storage ownership, handoff, and publication constraints as delivery. Its method references select diagnosis and repair instructions.
- `workflows/methods/diagnose.md` requires repository evidence, reproduction, observed versus expected behavior, the exact failed condition, the smallest responsible repair site, acceptance criteria, and unresolved evidence. Planning remains read-only and cannot compile.
- `workflows/methods/repair.md` requires inspection of that evidence, repair at its source, regression coverage, and preservation of unrelated work and frozen policy. The coordinator owns compilation, commit, and publication.
- Rust owns catalog loading, containment, supported-contract validation, and selection. Workflow-specific judgment belongs in method data. No dependency or service is added.

Generic catalog validation refuses duplicate or mismatched IDs, unsupported schemas or lifecycle values, missing methods, unsafe paths, and unsupported safety declarations. Catalog metadata cannot silently claim a runtime capability that the coordinator does not implement. Files have bounded sizes and must be ordinary contained files under the existing snapshot boundary.

## Durable state and compatibility

AllSource remains the authority for the selected workflow and immutable methods. Add optional, defaulted selection metadata to retained runs, omitted during serialization when absent. Legacy events must deserialize and reproduce identical serialized bytes and event digests. Existing histories default to `workflows/delivery.toml`; they are never rewritten.

Selection identity and definition path become immutable run scope. Verification reads the selected path rather than the current hardcoded delivery path. Changes to the selected definition or its methods invalidate the run's verification; changes to unrelated catalog entries do not redirect an accepted run. Recovery reads the retained selection, never chooses again from current catalog defaults.

Chronis retains the existing exact run-based task identity and claimant format so uncertain creation remains reconcilable. New task descriptions include the workflow ID without changing ownership rules. Prime continues reading historical delivery nodes; new source-linked plan and outcome records include workflow identity. Recall remains context and cannot select a workflow, authorize publication, or certify a regression test.

## Installed resources and existing repositories

Bundle the catalog, bug-fix definition, methods, indexed catalog guide, and repository-local catalog skill for both supported hosts. Keep existing delivery resources byte-identical. Rerunning `init` on a PR #3 repository adds missing catalog resources and appends its documentation link without replacing its delivery guide, skills, instructions, custom methods, or policy. Conflicting new managed resources block writes before mutation, using the existing descriptor-based setup writer.

Fresh initialization includes both workflows. Doctor reports catalog validity and missing resources without creating a run. Existing customized delivery files retain the established explicit-conflict behavior; this expansion supplies no overwrite or implicit migration mode.

## Verification and publication

Required proof covers selection through actual execution, not listing alone:

1. Focused Rust tests exercise discovery without store creation, both selections, unknown IDs, malformed catalog contracts, unsafe paths, and method/definition mutation. A legacy serialized event fixture proves byte-for-byte hash compatibility.
2. Application tests exercise the bug-fix methods through handoff, final-commit verification, publication readback, Chronis completion, and Prime attribution. Synthetic host or forge fixtures are labeled as such.
3. Install the built executable into an isolated prefix. With source-checkout reads denied, prove fresh initialization and additive initialization over the previously bundled delivery resources. Inspect actual generated files and run positive regression tests.
4. Use real authenticated Codex and Claude for a deterministic bug-fix request on this implementation branch. Retain reproduction evidence, both process receipts, selected workflow and method digests, handoff epochs, actual changed files, and positive executed test counts. Fix any missing layer within this PR.
5. Publish the implementation through one live feature branch and one PR. The exact final commit must pass required gates, and independent remote/PR head reads must agree. Read actual Chronis and Prime outcomes through their engines before reporting completion.

The authenticated journey may use this toolkit's own implementation PR as its publication destination. It must preserve the prepared catalog work and avoid creating a second proof repository or changing the existing example PR. New branch creation requires explicit user authorization before execution. If model execution is unavailable, record that limit and leave authenticated acceptance open rather than claiming completion.

All commands and gates remain bounded by 300 seconds, compiling gates remain serialized, and private stores and raw transcripts remain ignored. Run focused failures before broad gates; rerun combined checks after the final commit changes.

## Implementation sequence

1. Add catalog data, loader, CLI discovery, and explicit start selection with legacy fallback.
2. Bind immutable selection through run state, environment checks, Chronis description, and Prime records; prove old histories remain valid.
3. Bundle additive resources and indexed operator guidance; qualify the installed artifact.
4. Exercise authenticated diagnosis and repair, verify the complete candidate, publish one commit/PR, and add an indexed evidence report.

Completion means installed selection plus the real bug-fix journey reaches verified publication with task and memory readback. Merge, deployment, arbitrary graphs, review-only completion, hosted sync, and cross-machine transport remain outside this change.
