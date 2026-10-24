# Documentation

## Design

- [2026-10-24: Cross-host plan → build → verify → ship](plans/2026-10-24-cross-host-workflow-design.md) — product boundary, component ownership, state transitions, recovery, and implementation sequence.

## Reference

- [Cross-host workflow verification](reports/2026-10-24-cross-host-workflow-verification.md) — authenticated host runs, exact-commit gates, recovery observations, and supported limits.

- [First-release acceptance scenarios](reference/release-acceptance.md) — complete host journeys, failure cases, and required release evidence.
- [Forward handoff request](reference/handoff-forward-request.md) and [reverse handoff request](reference/handoff-reverse-request.md) — real-host fixture requirements.
- [CLI and recovery guide](reference/cli.md) — prerequisites, frozen verification policy, host handoffs, and publication recovery.
- [Delivery workflow definition](../workflows/delivery.toml) — versioned workflow data and method references.
- [Codex host guidance](../hosts/codex/SKILL.md) — managed Codex execution boundary.
- [Claude Code host guidance](../hosts/claude-code/SKILL.md) — managed Claude Code execution boundary.

The repository is an implementation preview. Source-level contracts and fixture definitions do not prove real host journeys. Acceptance criteria require retained runtime evidence before release.

- [Allstack delivery plan 9acd9993-3a70-4a87-b4fe-f62b96eb2cbb](plans/allstack-9acd9993-3a70-4a87-b4fe-f62b96eb2cbb.md)

- [Allstack delivery plan 750de2d1-ec0b-4327-be96-4f0e1ba5c5f5](plans/allstack-750de2d1-ec0b-4327-be96-4f0e1ba5c5f5.md)
