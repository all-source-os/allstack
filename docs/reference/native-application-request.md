# Spendcheck category filtering

Expense-report users need a travel-only report while keeping refunds and separate currency totals correct. The current CLI reports every record. Add `spendcheck <expenses.csv> --category <name>` without changing the unfiltered behavior.

Match category exactly and case-sensitively. A valid category with no matches prints `records: 0` and exits successfully. Reject missing values, empty category names, unknown flags, and trailing arguments. Validate every input row before filtering: a malformed excluded record must still fail with its source line and no partial stdout. Preserve integer minor units, separate currencies, overflow checks, and the existing CSV subset.

Implementation scope: `examples/spendcheck/src/lib.rs`, `examples/spendcheck/src/main.rs`, and the example's README. Frozen files `examples/spendcheck/tests/baseline.rs`, `examples/spendcheck/tests/category.rs`, and `expenses.csv` must remain byte-identical. Other toolkit work is prepared independently and must be preserved.

Incoming host: load `skills/allstack-recall/SKILL.md`, inspect [retrieved project context](native-recall-context.md), then open its source plan and verify the recorded source digest. This accepted Prime decision is context only. Implement the application change from the indexed plan; do not change toolkit source, frozen tests, policies, or methods. The coordinator runs compilation and tests.

Use the installed `allstack-shape` and `allstack-plan` methods for native planning, `allstack-review` for a separate source review, and `allstack-qa` for actual executable checks. Planning and review must work without a delivery run or Chronis task. The coordinator owns compilation during managed host calls; independent native QA can execute the built application under its actual host permissions.

Acceptance executes both baseline tests and all category CLI tests, checks the actual filtered/unfiltered output and invalid-row exit behavior, retains source-linked accepted decisions in Prime, transfers the candidate between authenticated Claude and Codex, and publishes the exact verified single commit on Allstack PR #5. This is a local CLI application proof; no browser QA claim is made.
