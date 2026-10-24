# Installed repository bootstrap verification

Date: 2026-10-24. Scope: `allstack init`, repository-local guidance, verification policy setup, and component inspection from an installed executable. [Design](../plans/2026-10-24-repository-bootstrap-design.md). [Implementation PR](https://github.com/all-source-os/allstack/pull/3).

## Installed artifact and isolation

The executable was installed into an isolated prefix using Rust 1.95 and `cargo install --path . --locked --debug --force --root <qualification-prefix>/install`. Its SHA-256 matched the executable produced by the final local test build:

```text
4e794ece7b2cd8ec06a99e8914cf811af6d3808be6081167bdb9685159d4d3d7
```

The compiled bootstrap test harness was copied outside the checkout. The installed executable and copied harness ran from the qualification prefix under a macOS sandbox that denied reads beneath the entire Allstack checkout, including its implementation worktree. A direct attempt to read the checkout's `Cargo.toml` raised `PermissionError` before qualification. The fixture repository was created outside that denied tree.

The sandbox profile allowed ordinary operations and added this restriction, with the actual checkout substituted for the placeholder:

```scheme
(version 1)
(allow default)
(deny file-read* (subpath "/absolute/allstack-checkout"))
```

The selected test was `installed_binary_initializes_real_chronis_and_executes_generated_policy` in `tests/bootstrap.rs`. `ALLSTACK_TEST_BINARY` pointed at the installed executable. The invocation used the copied harness, rather than a Cargo process that would need to read the denied source tree:

```sh
gtimeout -k 5 120 sandbox-exec -f <qualification-prefix>/source-denied.sb \
  <qualification-prefix>/bootstrap-tests --ignored --exact \
  installed_binary_initializes_real_chronis_and_executes_generated_policy --nocapture
```

The qualification environment put the user's installed host binary directory first on `PATH`. With inherited app precedence, the Claude version probe failed; its resolved absolute executable and the explicit installed-host precedence both returned `2.1.310 (Claude Code)`. No host authentication or permission settings were changed.

## Observed complete job

| Boundary | Observed result |
| --- | --- |
| `init --check --profile rust` | Ready; no target resources or project store created; existing source bytes unchanged |
| `init --profile rust` | Nine managed resources created and read back by digest; no conflicts or missing prerequisites |
| Chronis | Actual `cn 0.9.1` initialized the target `.chronis/`; `cn list --all --toon` read the project through its engine |
| Host guidance | Both repository-local skill links resolved to the installed CLI guide |
| Private storage | Git confirmed `.allstack/`, `.chronis/`, and `.prime/` ignored; setup did not create `.allstack/` |
| Doctor | `setup_ready: true`, `executables_ready: true`, `chronis: readable`, `stores_opened: false`, `authentication: not-probed` |
| Rerun | All nine resources unchanged; versioned bytes unchanged |
| Changed managed method | Initialization failed with a conflict and preserved its contents |
| Generated Rust policy | Formatting passed, Cargo executed one integration test, and whitespace checks passed |

The installed resources were the delivery workflow, two method documents, CLI guide, two host skills, verification policy, `.gitignore`, and documentation index. The generated policy digest was `9b39285fd803258d73450d1a2e1fcccd2a8f591dc53c4a684ec57431b80358a3`.

Retained output included:

```text
GATE format success=true
GATE tests success=true
running 1 test
test installed ... ok
test result: ok. 1 passed; 0 failed
GATE diff success=true
test installed_binary_initializes_real_chronis_and_executes_generated_policy ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out
```

The complete private qualification log has SHA-256 `505e97fceacef47cd5033a0987c6fb88a8249604b7c36fde30e1cda5fd115a52`. It includes the actual bootstrap resource digests, doctor response, Cargo execution output, and selected harness result. Zero-test library/doc units were also present; the positive integration-test summary supplied execution proof.

Doctor observed Codex CLI `0.163.0`, Claude Code `2.1.310`, Chronis `0.9.1`, Git `2.46.1`, GitHub CLI `2.101.0`, Cargo `1.97.1`, and rustc `1.97.1` on the qualified runtime `PATH`. The Allstack build and quality checks used the explicit Rust 1.95 toolchain; the generated fixture policy used that observed runtime Cargo/rustc pair.

## Safety and regression evidence

The default suite executed **29 tests**, all passing. Nine bootstrap tests cover real Git and executable process boundaries, with a temporary shell executable modeling only the Chronis protocol. They exercise existing instruction/index preservation, custom policy validation, effective ignore order, overridden ignores, dry-run engine deferral, preflight conflicts, missing prerequisites, traversal/symlink refusal, concurrent destination protection, and recovery after a failed Chronis initialization. The separate installed-artifact test above uses actual Chronis and is explicitly selected; the default suite's one skipped test is not counted as executed proof.

The four existing forward/reverse handoff fixture tests also passed locally. Formatting, Clippy with warnings denied, and dependency policy passed. No dependency or lockfile change was required; existing duplicate-version and unused-license-allowance warnings remain advisory.

[CI run 40124667431](https://github.com/all-source-os/allstack/actions/runs/40124667431) executed 29 tests plus the four handoff fixture tests on both Ubuntu 24.04 and macOS 14. Actual runner summaries were read from its logs. The source/dependency job passed formatting, Clippy, advisory, license, ban, and source checks. [PR checks](https://github.com/all-source-os/allstack/pull/3/checks) expose publication-SHA validation. CI visibility does not establish branch-protection enforcement.

## Supported limits

This qualifies fresh repository preparation and generated-policy execution. It does not establish host authentication, remote publication from the fresh fixture, hosted synchronization, or new model execution. The prior [cross-host verification report](2026-10-24-cross-host-workflow-verification.md) retains the actual Codex/Claude delivery evidence.

Existing Chronis engine validation is deferred in check mode to preserve its no-write contract. Apply and doctor validate through `cn`. Interrupted writes can leave completed resources that require inspection and a rerun. File creation refuses replacement; appends preserve existing bytes and compare the preflight contents, but do not create an authenticated boundary against arbitrary native filesystem mutation. The supported process and host scope remains sequential execution on one Unix machine.
