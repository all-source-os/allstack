# Harness Edition evaluation: gap matrix and build plan

Date: 2026-11-07. Scope: the Harness Edition proposal compared with the Allstack repository, the decebal-claude-skills checkout, and the local host configuration. This report records what was checked, what was built in this repository, and what remains open.

## Headline

The proposal says the `pr-guard` hook is not wired. On the machine where this was checked, `claude-guard pr-guard` is wired in `~/.claude/settings.json`, and all five claude-guard hooks are wired. The real gaps are the template repository, a stale local checkout, skill drift, and the missing spec gate. The approval-before-beads loop was also a real gap: nothing in code enforced it. This repository now provides the spec gate and the lifecycle skills. Guard hooks, the `skillsync` installer, and the beads-side enforcement are not in this repository.

## 1. Gap matrix

Status is one of `have`, `part`, `gap`, `conflict`, `water`, or `unverified`. Evidence cites file and line where the check was made. The `Home` column names the repository that owns the fix.

| Proposal item | Status | Evidence | Home |
| --- | --- | --- | --- |
| Seven stages, one skill each | part | allstack `skills/` held shape, plan, review, qa, ship, recall, handoff. No Think, Spec, or Reflect names. | both |
| Think: brainstorm to concepts_brief.md | part | decebal `skills/brainstorming`. allstack-shape writes an indexed brief with no `concepts_brief.md` name (shape SKILL.md:27). | neither |
| Spec: design draft plus Goldfish review | gap | No egm-* skills in allstack, `~/.claude/skills`, or `~/.codex/skills`. Plan review was same-context (allstack-plan SKILL.md:8, 12-25). | allstack |
| Plan: claude-prd to claude-beads to cn | part | claude-prd and claude-beads exist in decebal (symlinked). allstack-plan creates no cn task (SKILL.md:8, 33). | both |
| Build: cn ready loop or cos spawn | part | allstack-delivery through the allstack CLI (hosts/codex/SKILL.md:12-20). No standalone cn-ready skill. | allstack |
| Review: prove-it and security-review, fresh context | part | allstack-review had no fresh-context rule (SKILL.md:8, 21). security-review lives in decebal. | allstack |
| Ship: pr-proof, one PR one commit, pr-guard | part | allstack-ship single-PR reuse (SKILL.md:8, 22). pr-proof was HTML-only through a `~/.agents/skills` symlink. pr-guard wired (settings.json:135). | allstack |
| Reflect: brief, scorecard, drift report | gap | No retro or learn skill in allstack skills or METHODS (skills.rs:5-25). allstack-recall covers recall only. | allstack |
| File hand-offs between stages | part | Docs and Prime memory, no typed manifest. allstack hardcodes `docs/plans/allstack-{run.id}.md` (application/mod.rs:184). | allstack |
| Entry skill `harness` routing stages | gap | No harness skill in either repository or installed directory. | both |
| gstack: office-hours, plan-ceo-review, autoplan | part / part / gap | Installed gstack symlinks only. decebal has no plan-ceo-review or autoplan directory. | decebal |
| gstack: plan-eng-review, review, ship | have | allstack-plan:12-33, allstack-review, allstack-ship, plus gstack symlinks. | allstack |
| gstack: investigate | part | `workflows/methods/diagnose.md` was not in METHODS (skills.rs had seven entries). decebal has systematic-debugging. | allstack |
| gstack: qa and browse | part | allstack-qa named no browser driver (SKILL.md:8, 16, 28). Driver choice is open. | allstack |
| gstack: cso | part | decebal security-review. allstack had no security method. | both |
| gstack: careful, guard, freeze | have | Hook-bound at settings.json:115 (infra-guard). | decebal |
| gstack: retro and learn | gap | Same as the Reflect row. | allstack |
| gstack: captain | gap | No match in the gstack install or in allstack. The mapping to cos is unverified. | none |
| Glossary `docs/concepts/agentic-harness.md` | gap | Grep for concepts/agentic-harness, Elephant, and Goldfish returned no hits in allstack or decebal. | both |
| Confidence scale rock/sand/water | have | allstack-shape SKILL.md:25. | allstack |
| semantic-isa for agent specs | part | `~/.claude/skills/semantic-isa` (115 lines, design and write only). Not vendored in decebal. | decebal |
| Human approval gate before beads | gap | accept_plan set Building with no approval input (application/mod.rs:54-61). claude-prd had no sign-off step. claude-beads said human actions never block (SKILL.md:29-52). | both |
| Fresh-context review, Claude Code | part | A spawned process gives freshness (hosts.rs:51-73). No hook checks subagent identity. claude-guard source grep returned no match. | both |
| Fresh-context review, Codex | part | `codex exec --sandbox read-only` (hosts.rs:33-48). `~/.codex/hooks.json` has no agent matcher. | both |
| Guard surface, Claude | have | settings.json:115, 125, 135, 145, 189. | decebal |
| Guard surface, Codex | have on trunk, part in source | `~/.codex/hooks.json` `^Bash$` runs codex-guard pr-guard. The pr-guard source for codex-guard exists only on `feat/codex-parity`. | decebal |
| claude-guard source has pr-guard | have on origin/main, part locally | origin/main commit 0514154 adds `pr_guard.rs`. The local checkout on `feat/prompt-id` lacks it (main.rs:39-45). | decebal |
| Template configs lack pr-guard | gap | `configs/settings.json` and the `hooks/README.md` install block do not list pr-guard. | decebal |
| prompt-number matcher | gap | settings.json:141 uses `Write\|Edit`. prompt_number.rs:92 also handles MultiEdit. | both |
| Prod-guard tokens | gap | `~/.claude/prod-guard-tokens.txt` still holds the template value (`prod:my-company-prod`), so the prod layer matches nothing. | allstack (doctor) |
| Fail-open policy table | have, partial | infra/mod.rs:144-160 has Allow, Ask, and Deny. prompt-number denies (prompt_number.rs:90-99). The proposal's claim that every guard fails open is wrong. | decebal |
| Sandbox kit wires guards | gap | `docker-sandbox-team-kit/base/claude/settings.json` has no claude-guard entry. | both |
| Idempotent hook installer | gap | No hook writer in allstack. decebal installs by hand (hooks/README.md:31-38). | decebal |
| Process ownership | have | allstack process.rs:51-68, 106-114, 116-161. | allstack |
| Build lock | have | allstack build_lock.rs:10-53 and runtime.rs:356-359. | allstack |
| Gate runner | part | gates.rs is a parser only. runtime.rs:353-392 is the runner. | allstack |
| cn task syntax in claude-beads | drift | Repository SKILL.md uses `cn create` (lines 225 and others) and `cn sync --flush-only` (line 432). `cn create --help` returned "unrecognized subcommand 'create'". The installed copy uses `cn task create`. | decebal |
| claude-beads installed copy | drift | `~/.claude/skills/claude-beads/SKILL.md` sha 3b524961, repository sha fe5a014b. claude-prd is a symlink whose sha 686f48d4 matches. | decebal |
| Dangling claude-cn-* symlinks | unverified | Symlinks exist under `~/.claude/skills`. Target existence was not checked. | decebal |
| Installer for allstack skills, both hosts | have | skills.rs:47-63 emits `.agents/skills` and `.claude/skills`. bootstrap.rs:280-300 checks and applies with conflict reporting. README.md:23-24. | allstack |
| Per-repo install versus global | gap | allstack install is per repository. The global `~/.claude` path is separate. | both |
| Chronis adopt, start, current | part | presentation.rs:68, 82, 244. `current` reads the run store, not cn. | allstack |
| Team retro from cn history | water | No event query in `cn --help`. `cn list --all --format json` is valid (`cn list --help`). | decebal |
| cos: judge, --check, --scope, verdict | part | Installed binary help lists `judge`, `--check`, `--scope`, and `scorecard`. Source has none (grep returned no match). | neither |
| cos: COS_AGENT_SLOTS | part | String present in the installed binary only. No slot logic in source. | neither |
| cos: spend cap and deadline | have | agents/harness.rs:28 `--max-budget-usd`. agents/mod.rs:31 CEILING 2h. | neither |
| cos agents: three versus one brief | conflict | main ships three agents. `feat/cos-dashboard-from-upstream` was not read. | neither |
| Installed skill and agent drift | gap | `~/.claude/agents/strategist.md` and `tactician.md` hardcode a local chief-of-staff path. | decebal |
| Browser driver for qa in sbx | water | sbx is not installed. The cos sandbox is Seatbelt, not sbx. | neither |
| EGM install, plugin or vendor | gap | `claude plugin marketplace add google/elephant-goldfish-model` (harness.txt:277). EGM was not fetched. | both |

## 2. Build plan

Each entry names its confidence tier and what would move it up. Tiers are rock (bet it), sand (shape it first), and water (spike it first).

### Installer path: one command per repository for both hosts

- Extend `init --skills-only` to ship the harness skills. Both hosts already come from one METHODS list. Add `harness`, `spec`, `reflect`, and `investigate`. Confidence: rock for the mechanism, sand for the skill content. Moves up after one real feature run through the harness.
- Add `allstack doctor` checks as read-only probes. The probes would run each guard subcommand from settings.json and hooks.json with stdin closed and fail on usage text. They would fail when prod-guard-tokens.txt equals the template, and check `cn <sub>` tokens in SKILL.md against `cn <sub> --help`. Confidence: rock, because every check is a pure read.
- User-level hooks and global skills: a Rust binary in decebal, named `skillsync`, with `check`, `install --dry-run`, and `doctor`. It would merge guard entries into settings.json and hooks.json idempotently by command string, with a backup, and compare sha values between source and `~/.claude/skills` and `~/.agents/skills`. Confidence: sand. The Codex hooks.state trust-hash behavior is unverified. Moves up when a test round-trips both files.

### Spec layer

- Skills `allstack-spec` (this repository) and the EGM skills in decebal. Confidence: sand. Moves up when EGM's reviewer configuration and its five review stages are read. Upstream was not fetched.
- `allstack spec review`, a subcommand that starts a read-only host process with only the draft path and repository root. Confidence: rock for the spawn, which reuses the host argv in `hosts.rs`. Water for proving the reviewer has no author context. No verified signal on either host shows the author context. Spike: whether Claude Code exposes agent identity to hooks, and whether Codex does.
- An approval record bound to the design digest, checked by `spec check`. Confidence: sand. The gap itself is rock. The digest binding is a design choice, recorded in the approval semantics above.
- Enforcement in claude-beads: it must call `allstack spec check` before it cuts beads, and a claude-guard PreToolUse deny when the approval is missing. Confidence: sand. Not in this repository.
- Glossary `docs/concepts/agentic-harness.md`, indexed in the same change. Confidence: rock. Moves up with one peer review session.

### Ledger layer

- Fix claude-beads: port the installed copy's `cn task create` form into the repository copy, and add a lint that checks each `cn <sub>` against `cn <sub> --help`. Confidence: rock.
- Bridge allstack to epics: read ready beads rather than one task per run (chronis.rs:119-155). Confidence: sand. Allstack's lifecycle is run-level today.
- Team retro: spike only. Confidence: water. Moves up with a measured listing of events across the three local `.chronis` stores through cn. The path flag is absent, and cn is bound to the working directory.

### Guards layer

- Add pr-guard to `configs/settings.json` and the `hooks/README.md` install block. Change the prompt-number matcher to `Write|Edit|MultiEdit`. Add a deny, ask, and allow matrix. Confidence: rock.
- Merge or rebase origin/main into the decebal checkout so `pr_guard.rs` is present locally. Confidence: rock, verified by the merge-base check. The merge-base check had no printed output in the first run, and the exit code was taken from a second run.
- Wire claude-guard into the sandbox kit `base/`. Confidence: sand. Sbx was not run and egress was not tested. Moves up with `sbx kit validate` and one sandbox run.
- A kernel that admits or refuses semantic-isa operations. Confidence: water. Spike one agent first.

### cos layer

- Land judge, --check, --scope, and verdict in source, matching the installed binary's help text. Confidence: sand. The judge behavior was never run, and source and binary may differ.
- Merge or close `feat/cos-dashboard-from-upstream`. Confidence: rock. Owner decision.

### Distribution

- Skill-sync check covering sha, symlink versus copy, and dangling links. Confidence: rock.
- Vendor or plugin decision for EGM and the club skills. Confidence: sand. A local directory marketplace works without network access through `extraKnownMarketplaces` in settings.json.

### Ordered first bet

The smallest set of rocks that makes the install verifiable end to end. The harness router and the EGM spec gate remain sand and stay outside this set.

1. Merge origin/main into the decebal checkout and confirm `pr_guard.rs` is present locally.
2. Add pr-guard to `configs/settings.json` and `hooks/README.md`, and fix the prompt-number matcher.
3. Fix the claude-beads `cn` syntax in the repository copy and add the `cn <sub> --help` lint.
4. Add `skillsync check`, or the allstack doctor equivalent: sha comparison, dangling links, and the template token check.
5. Land the glossary, indexed.

Acceptance for this set: `skillsync check` and `allstack doctor` both exit 0 on the machine, `claude-guard pr-guard` exits 0 with stdin closed, and the claude-beads lint passes.

## 3. What this repository now implements

- Skills `allstack-harness`, `allstack-spec`, `allstack-reflect`, and `allstack-investigate`, registered in METHODS. `allstack init --skills-only` installs them into `.agents/skills` and `.claude/skills`. Skill count went from seven to eleven.
- `allstack spec review <path> [--host codex|claude]`. It starts a read-only host process whose prompt contains only the draft path and repository root. It writes a receipt under `.allstack/spec/reviews/` and prints the receipt path, digest, verdict, and findings. A host failure exits nonzero and writes no receipt.
- `allstack spec approve <path> --by <name> [--without-review]`. It records the digest, approver, and receipt path as an event in the AllSource store. Without `--without-review` it refuses when no receipt exists for the current digest.
- `allstack spec check <path>`. It exits 0 only when the latest approval matches the current digest and its receipt, if any, still exists. Otherwise it exits nonzero with `unapproved`, `changed since approval`, or `unreviewed`.
- Tests in `tests/spec.rs` use a fake `codex` executable on `PATH`. They cover receipt creation, the reviewer prompt contents, approval binding, edit invalidation, refusal without review, receipt removal, and host failure.

## 4. Not checked

- The claude-web-club repository is absent from this machine. Proposal items about it were judged from the proposal text only.
- EGM and gstack were not fetched. The gstack install was checked by grep only, and no SKILL.md body was read.
- Not read: claude-prd and claude-beads bodies (checked by sha only), security-review, systematic-debugging, the installed semantic-isa body, and codex-prd.
- The fake-host tests do not exercise a real Codex or Claude Code process. The reviewer's read-only sandbox and its absence of author context were not observed on a real host.
- Claude Code subagent identity in PreToolUse hooks: no documentation was read.
- Codex `~/.codex/hooks.json` contents were read by a verifier, not in this session. Hook output schema and trust-hash behavior were not verified.
- cos source was checked by grep only. The installed judge and scorecard behavior was never run. COS_AGENT_SLOTS was not verified as live. A machine-wide grep for COS_AGENT_SLOTS timed out at 120 seconds in the verifier run and did not complete.
- Sbx is not installed. The sandbox kit was read as files only.
- Prod-guard-tokens.txt state came from a verifier read, not a re-read.
- The merge-base check on commit 0514154 printed no output in the first run. Its exit status was taken from a second run.
- `cargo deny check` was not run in this session. No dependency changed, so the lock file is unchanged.
- The `skillsync` binary, the claude-beads enforcement, the claude-guard deny rule for missing approvals, and the glossary's Plan-stage enforcement are outside this repository and were not built here.
