# Installed CLI repository bootstrap

Status: accepted implementation design on `feat/repository-bootstrap`.

Confidence: rock. The existing runtime and actual host journeys are proven; the missing path is preparing a different repository from the installed executable without copying files from the Allstack checkout.

## Outcome

A developer installs Allstack, enters an existing Git repository, and runs `allstack init` with an explicit supported verification profile or an existing verification policy. The executable installs its bundled workflow methods and repository-local guidance for both hosts, indexes documentation, ignores private runtime stores, and initializes Chronis. `doctor` identifies remaining prerequisites. No hosted AllSource account or Allstack source checkout is required.

Bootstrap never creates branches, commits, pushes, publishes, authenticates hosts, or installs global skills. Starting a managed run still requires the existing authority and branch checks.

## Approach

1. **Bundled Rust bootstrap command, recommended.** Ship authored templates with the executable using compile-time resources. Preflight every destination before writing. This reuses the installed CLI, path safety checks, bounded process supervisor, and existing workflow data.
2. **Downloadable template archive.** Keeps the binary smaller but requires a second artifact, version matching, and a separate conflict-safe installer.
3. **Documented manual copy.** Adds no runtime behavior but retains the missing installed-user path and leaves host-guidance links dependent on their original checkout layout.

Use the first approach. Add no dependency or configuration layer unless the accepted behavior requires one.

## Interface and ownership

- `allstack init --check` returns the proposed files, missing prerequisites, and conflicts without repository writes or Chronis initialization.
- Applying initialization requires an explicit Rust/Cargo test profile or an existing valid verification policy. Do not guess a project's required checks or emit a test gate that cannot establish execution.
- Preserve existing custom policies and documentation. Refuse conflicting managed templates before applying the batch. Identical files are an idempotent success.
- Install workflow data and methods under `workflows/`, the CLI reference under indexed `docs/reference/`, and guidance under repository-local Codex and Claude skill directories. Installed links must resolve from those locations.
- Append only missing private-store ignore rules and documentation index entries. Preserve unrelated bytes and existing repository instructions.
- Initialize Chronis through `cn init --toon` only after successful preflight, and only when the project store is absent. Surface uncertain outcomes and support rerunning initialization.
- Report template version/digests and actual created, unchanged, appended, or conflicting paths. Do not record initialization as a successful delivery run.

Presentation parses bootstrap arguments. Infrastructure owns bundled resources and filesystem/process effects. Bootstrap does not open the mutable run engine before preflight; dry-run process evidence must not create an unignored `.allstack/` inside the target. The existing run domain and transition history remain unchanged.

`doctor` reports setup completeness separately from executable availability and authentication. Read actual files, policy validity, ignore behavior, Chronis presence, and component state. Executable version probes continue to mean availability only.

## Safety and recovery

Reject path traversal and symlink destinations. Preflight all managed paths, then use guarded atomic writes that cannot overwrite a file introduced after preflight. A partial filesystem or Chronis failure must name completed and remaining work; rerunning converges without overwriting user changes. Never add a force-overwrite or permission-bypass mode to make a fixture pass.

The target may have existing documentation, ignore rules, and policies. Initialization must work there without replacing user-owned content. Configuration changes remain reviewable and uncommitted until the developer follows their repository's normal Git process.

## Acceptance and proof

- An executable installed into an isolated prefix initializes a fresh Rust fixture repository without access to the Allstack source tree at runtime.
- Check mode creates no repository files; conflicts preserve destination bytes and do not initialize Chronis.
- Applying initialization creates complete indexed resources, valid policy data, correctly located host guidance, ignored runtime storage, and a real Chronis project store.
- A second initialization is idempotent. A changed managed file is preserved and reported as a conflict. Symlink/path-escape attempts fail.
- `doctor` reports actual missing prerequisites before initialization and setup readiness after it, without claiming host authentication.
- Black-box tests invoke the real CLI and real Git/Chronis boundaries where available. Retain an indexed report with installed binary digest, exact commands, and observed results.
- Existing runtime tests, the four handoff fixture tests, formatting, Clippy, dependency checks, and Linux/macOS CI remain green on the one implementation commit.

Deliver through one new PR from `feat/repository-bootstrap`, based on current `main`, after explicit branch permission. No release tag, package-registry publication, new catalog workflow, or merge is part of this continuation.
