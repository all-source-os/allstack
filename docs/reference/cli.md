# CLI and recovery guide

Status: implementation preview. This guide describes the source interface. The [release acceptance scenarios](release-acceptance.md) track the evidence required to establish supported end-to-end behavior.

## Prerequisites

Use one Unix machine with Git, GitHub CLI, Rust/Cargo, Chronis `cn`, Codex CLI, and Claude Code available on `PATH`. Authenticate both hosts and `gh` through their normal setup. The preview supports `github.com` remotes over HTTPS or SSH; GitHub Enterprise and other forges are outside its current scope.

Install from the reviewed Allstack checkout with `cargo install --path . --locked`. The Rust package requires Rust 1.95 or later. `allstack --help` and each subcommand's `--help` expose the installed interface.

Native command evidence is available without managed delivery: `allstack check --expect-exit 4 --seconds 120 -- /absolute/program arguments`. Omit `--expect-exit` to expect zero; `--input prompt.txt` supplies up to 128 KiB of UTF-8 stdin. The 2..270 second deadline includes source metadata and execution. Ignore `.allstack/` first. JSON returns the receipt location, actual outcome, executable identity, source checkout, and separate raw stream paths/digests. A matching expected nonzero exit succeeds; timeout, signal, changed executable, spawn failure, and unexpected status fail and retain a private receipt. No task, run, or Prime store is created. A binary digest identifies observed bytes; establish its source build independently.

Put the installed executable directory on the `PATH` inherited by both coding hosts. A custom `cargo install --root <prefix>` needs `<prefix>/bin` there as well. Keep the selected policy environment stable from verification through shipping. The [fresh-repository verification](../reports/2026-10-24-fresh-repository-delivery-verification.md) records the complete authenticated journey and actual destination readbacks.

Prepare the target repository before starting:

1. Use an existing feature branch whose creation the user authorized. Confirm its PR has not merged. Allstack does not create branches.
2. Inspect bundled setup with `allstack init --check --profile rust`, then apply with `allstack init --profile rust`. The explicit Rust profile requires `Cargo.toml` and `Cargo.lock`. For existing verification data, use `allstack init --policy <repository-relative-path>` instead. Check mode creates no target files or project store.
3. Initialization installs `workflows/delivery.toml`, referenced methods, `docs/reference/allstack.md`, its documentation index entry, and repository-local skills in `.agents/skills/allstack-delivery/` and `.claude/skills/allstack-delivery/`. It appends private-store ignores and initializes Chronis with `cn init --toon` only when the target store is absent. Existing custom policies, instructions, and documentation remain intact. Different managed resources are conflicts; resolve them explicitly before rerunning.
4. Review the policy described below against the repository's required checks. Add and index a request document. Commit the prepared inputs through the repository's normal authorized process so the starting tree is clean. Initialization does not run gates, create a branch, commit, publish, authenticate hosts, or install global skills. Partial failures name completed paths; inspect their effects before rerunning. There is no force-overwrite mode.
5. Ensure repository setup works in a new detached worktree. Gitignored dependencies and build output are not included in source snapshots. Prepare required local inputs explicitly before invoking managed work.

Run `allstack -C /path/to/target-repository doctor` (add `--policy <path>` for a custom policy). Its JSON separates `setup_ready` from `executables_ready`, reports missing/conflicting resources, actual ignore behavior, tracked private files, Chronis engine readback, and tool versions. It does not open mutable AllSource/Prime run stores. A successful `--version` probe establishes executable availability; it does not establish authentication, model access, or host behavior.

`init --check` returns `ready: false` with planned paths, digests, conflicts, and missing prerequisites when setup cannot be applied. Existing Chronis engine readback is explicitly deferred until apply; opening its engine could write storage metadata. Check-mode readiness covers resource and executable preflight, with that readback still outstanding. Applying failed preflight stops before writing managed resources or initializing Chronis. Symlinks, traversal, reserved policy destinations, and nonregular/multiply-linked files are refused. File creation cannot replace a destination introduced after preflight. Index/ignore updates append bytes after comparing the original contents. Concurrent edits can require inspection and a rerun.

## Workflow selection

Use `allstack catalog` to list repository workflows or `allstack catalog bugfix` to inspect evidence-led diagnosis and repair. Pass `start --workflow bugfix` to select it; the default remains `delivery`. See the [catalog reference](workflow-catalog.md) for contracts and additive initialization. Catalog discovery opens no mutable run store. Existing repositories without a catalog and retained runs without selection metadata preserve legacy delivery behavior.

## Frozen verification policy

Create a repository-relative TOML file such as `workflows/verification.toml`:

```toml
command_seconds = 120
environment = ["RUSTUP_TOOLCHAIN"]

[[gates]]
name = "format"
argv = ["cargo", "fmt", "--check"]

[[gates]]
name = "tests"
argv = ["cargo", "test", "--locked"]
success_marker = "test result: ok."
test = true
compile = true
```

Commands are argv arrays, not shell expressions. Every policy needs at least one executed-test gate with a nonempty success marker. Test gates require a positive Cargo or nextest execution count in the retained output; zero tests, `cargo check`, `clippy`, and `--no-run` cannot satisfy that requirement. Other runner formats currently remain inconclusive.

Set `compile = true` for every gate that can compile. Such gates acquire the machine build lock and check competing compiler processes. Allstack does not terminate another session's work to obtain capacity.

The request, workflow definition, method text, and policy are retained by digest at run start. Verification and shipping check the current workflow, methods, and policy against the accepted versions. Relevant environment fingerprinting includes the running Allstack executable digest, OS, architecture, selected environment variables, and Git/Rust/Cargo/Chronis versions. This is an explicit fingerprint, not a hermetic environment guarantee.

The policy permits `command_seconds` in `1..=300`; managed execution clamps work to 290 seconds, leaving cleanup within the 300-second ceiling. An outer guardian also bounds the entire CLI command, so a sequence of individually bounded subprocesses cannot accumulate an unlimited command lifetime. Parent ownership uses a guardian and an owner pipe so caller death revokes owned process lifetime. Host operations and gate commands remain subject to normal host permissions and repository hooks.

## Codex → Claude Code → Codex

The following sequence assumes request and policy paths from the preceding setup. Substitute the returned run ID. Read each result before issuing the next command; epochs below describe an uninterrupted first handoff sequence.

```sh
allstack -C /path/to/target-repository start \
  --request docs/requests/change.md \
  --policy workflows/verification.toml \
  --host codex --allow-commit --allow-publish
allstack -C /path/to/target-repository plan <run> --epoch 1
allstack -C /path/to/target-repository handoff <run> --epoch 1 --to claude-code
allstack -C /path/to/target-repository resume <run> --epoch 2
allstack -C /path/to/target-repository build <run> --epoch 2
allstack -C /path/to/target-repository handoff <run> --epoch 2 --to codex
allstack -C /path/to/target-repository resume <run> --epoch 3
allstack -C /path/to/target-repository verify <run> --epoch 3 \
  --message "feat(example): implement accepted change"
allstack -C /path/to/target-repository ship <run> --epoch 3 \
  --title "feat(example): implement accepted change"
```

Only pass `--allow-commit` and `--allow-publish` when trusted user instructions authorize those actions for this target. These flags record supplied authority; they do not establish that the caller obtained it. Repository rules and host permissions still apply.

`start` creates and claims a Chronis task, reads back ownership, and restores a run-owned detached worktree. `plan` invokes the active host, writes its plan to `docs/plans/allstack-<run>.md`, and indexes that document in `docs/README.md`. `build` invokes the selected host in the active worktree and retains the resulting checkpoint. Handoff restores another detached worktree, increments the epoch, and reconciles the Chronis claimant. It does not copy a conversation transcript.

`resume` returns authoritative run state, request, plan, and contextual Prime recall. It does not execute a host or advance a stage. `status <run>` returns run state; `history <run>` returns validated event history. Always use the current returned epoch, especially after recovery.

`evidence <run>` reads digest-validated gate receipts and run state, including completed runs whose worktrees were removed. `recall <query>` reads source-linked contextual Prime nodes. These diagnostic commands expose local private data; publish only reviewed excerpts. They do not mutate a run or certify anything beyond their recorded source evidence.

The reverse sequence starts with `--host claude-code` and hands off to `codex` for implementation. It uses the same transition and verification contracts. Each direction requires its own real-host evidence.

## New files and checkpoints

In the active worktree reported by `status`, explicitly stage every new intended file with `git add -- <path>`. Then `checkpoint <run> --epoch <epoch>` retains the workspace. Untracked files block snapshot creation; Allstack never silently includes them. Checkpointing or handing off invalidates previous verification.

Snapshots retain tracked contents and executable modes, including staged new files and tracked deletions. They reject symlinks, path escapes, recognized sensitive filenames, files above 4 MiB, totals above 64 MiB, and manifests above 10,000 files. Filename exclusions are a limited guardrail; review the candidate for secrets independently.

Old worktrees remain separate after handoff. Native edits there do not change the incoming snapshot. An old epoch cannot drive Allstack-managed mutation. Local users and native host tools remain capable of operating outside Allstack; no filesystem sandbox or authenticated per-host identity is claimed by the epoch number.

## Interruption and uncertainty

Inspect `status` and `history` before retrying a failed command. A pending operation means its outcome requires reconciliation.

| Pending operation | Supported next action |
| --- | --- |
| Task setup | `recover <run> --epoch <epoch>` reconciles task identity and ownership |
| Host planning or building | Inspect the active worktree and private process evidence, then `recover`; recovery checkpoints current edits without replaying the uncertain host call |
| Handoff | `recover` completes the recorded transfer and reads back Chronis ownership |
| Plan recorded with `memory_pending` | `recover` retries Prime indexing without invoking the planning host again |
| Commit or verification | Repeat `verify` with the current epoch and authorized commit message; it reconciles the candidate and reruns final gates |
| Publication or later task/memory update | Repeat `ship`; destination state determines remaining work |

After recovery of an uncertain planning call, planning remains necessary unless an accepted plan was recorded. Inspect preserved edits before deliberately invoking `plan` again. A command that failed after recording its result may already have advanced state; always read the destination state first.

No retained checkpoint can prove that later abandoned edits were durable. Missing or corrupt objects block recovery. Chronis claimant mismatch blocks managed execution. Prime recall supplies context only; source-linked nodes cannot authorize a transition or certify a successful check. Unavailable Prime storage degrades recall while retaining authoritative run access; pending memory indexing remains explicit until reconciled.

## Verification and shipping

`verify` materializes the accepted checkpoint into the original feature checkout and creates or amends the authorized owned commit. It then executes frozen policy gates on that exact SHA and checks for changed content or environment. Do not edit or switch the original checkout during this operation. Branches containing another author's commits or more than one existing feature commit require explicit handling before the run.

`ship` checks candidate cleanliness, workflow/policy/method digests, environment, branch liveness, remote identity, and verification. It publishes the exact SHA with a branch-specific lease, reads the remote ref, reuses or creates one PR, and reads back its open state and head SHA. A lost response requires destination reconciliation rather than an assumed retry.

Chronis completion and Prime outcome indexing follow confirmed publication. If either later write fails, rerunning `ship` reconciles remaining metadata without treating missing metadata as proof that publication failed. `publication` fields expose those component outcomes. `shipped` covers the remote commit, open PR, completed task, and indexed outcome; merge and deployment are separate operations.

Verification records the prepared commit so a changed checkpoint can be committed and verified again. Commit receipts carry the run identity for reconciliation after a lost acknowledgement. After shipping, Allstack removes its detached worktrees only when their content and base still match their last accepted snapshots. Unexpected edits stop cleanup and preserve the workspace for inspection. Repeating `ship` with the current epoch retries cleanup without publishing again.

## Private evidence and public proof

`.allstack/` holds AllSource events, Prime graph state, object snapshots, run worktrees, and process receipts. It is local-only and must remain ignored. Read component stores through their APIs or Allstack commands, never by scraping WAL or storage segments.

Retain tool versions, run IDs, exact commits, actual test counts and outputs, interruption points, remote/PR readbacks, and private artifact digests when validating the release. Publish only deliberately reviewed evidence in indexed repository documents. Fixture definitions and source tests do not establish that authenticated Codex and Claude Code completed the real journey.

Initialization and doctor permit Chronis’s tracked `.chronis/sync/events.jsonl` Git sync export. Other tracked files under `.chronis/`, `.prime/`, and `.allstack/` remain rejected as private state. Existing exports are preserved.

Publishing changes under `.github/workflows/` requires GitHub workflow-write authority. A repository-scoped OAuth token can read the repository and create a PR while still failing the Git push for those files. Use an existing authorized SSH identity for the same repository, or a token with the required scope. Preserve the verified commit and reconcile the actual remote branch before retrying publication.
