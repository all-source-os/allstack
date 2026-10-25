Write/ExitPlanMode disabled. Plan delivered as message artifact below.

---

# Spendcheck `--category` filter — engineering plan

## Context

Expense-report users need a travel-only report while refunds and separate currency totals stay correct. Today the `spendcheck` CLI reports every record; there is no way to scope a report to one category. The accepted change adds `spendcheck <expenses.csv> --category <name>` with exact, case-sensitive matching, leaving the unfiltered path byte-identical.

Confidence: **rock**. The contract is already pinned by frozen tests (`tests/category.rs`, `tests/baseline.rs`) and `docs/reference/native-application-request.md`. Standard library only; the change is one validation loop plus the argument grammar.

Bounded to the application change. The separate source review, executable QA run, Prime decision retention, the Claude↔Codex candidate transfer, and PR #5 publication are owned by their own methods and are **not** part of this plan.

## Complete job (shaped)

A human runs the CLI against the strict three-column CSV (`category,currency,amount`) and gets a report scoped to one category, with correct per-currency, refund-aware totals.

- **Trigger:** `spendcheck <expenses.csv> --category <name>`.
- **Input:** the existing CSV subset — unquoted fields, uppercase three-letter currency, signed decimal amount with exactly two fraction digits.
- **Output:** `records: N\n` then per-currency totals sorted by currency, counting and summing only rows whose category equals `<name>` exactly.
- **Failure behavior:** every row is validated before any filtering; a malformed row fails with its source line and no partial stdout, even when the filter would exclude it. Missing value, empty name, unknown flag, and trailing argument are rejected. A valid category with no matches prints `records: 0` and exits 0.

## Observed current behavior

`src/lib.rs`
- `report(input: &str) -> Result<String, String>` (lib.rs:3). Checks the header; per row validates field count (3), non-empty trimmed category (lib.rs:12), 3-char ASCII-uppercase currency (lib.rs:16), and amount via `minor_units` — exactly two fraction digits, i64 minor units, `checked_mul`/`checked_add`/`checked_neg` (lib.rs:38–58).
- Accumulates per-currency totals in a `BTreeMap<&str, i64>` with `checked_add` plus an overflow error (lib.rs:22); counts every valid row.
- Emits `records: {count}\n` then `{CCY} {sign}{whole}.{frac:02}\n` per currency in `BTreeMap` (currency-sorted) order (lib.rs:25–34). An empty map prints only the count line.

`src/main.rs`
- Accepts exactly one argument (the path); any other count yields `usage: spendcheck <expenses.csv>` (main.rs:3–9). On `Ok` it `print!`s the whole report; on `Err` it `eprintln!`s and `exit(1)` (main.rs:10–16). stdout is written once, after the full report string exists — so "no partial stdout" is already structural.

Frozen contract (stays byte-identical): `tests/baseline.rs` calls `spendcheck::report(input)` with a **single argument** (baseline.rs:4,11), so the `report` signature cannot change. `tests/category.rs` drives the **binary** and fixes every observable string (`records: 3\nEUR 10.00\nUSD 7.00\n` for `--category travel`; `records: 0\n` for `rent` and for `Travel`). `expenses.csv` is the shared fixture.

## Alternatives considered

- **Change `report`'s signature to take the category** — rejected. Frozen `baseline.rs` calls `report(input)` with one argument; adding a parameter breaks a frozen file's compilation.
- **Duplicate the parse/validate loop in a second function** — rejected. Validation (currency, amount, field count, overflow) is the correctness core; duplicating it invites skew between filtered and unfiltered paths.
- **Chosen:** keep `report(input)` as a thin wrapper and add one new public function carrying an optional category. Single validation loop, frozen signature preserved, extends the existing seam rather than introducing an abstraction.

## Proposed behavior and exact files

### `examples/spendcheck/src/lib.rs`
- Add `pub fn report_filtered(input: &str, category: Option<&str>) -> Result<String, String>` holding the current loop body.
- Reduce `report` to `report_filtered(input, None)` — preserves the frozen one-argument signature and behavior.
- Validation stays **unconditional, in the same order as today**, computed for every row before any filtering. Only the count and total accumulation become gated:
  ```rust
  if category.map_or(true, |wanted| fields[0] == wanted) {
      let total = totals.entry(currency).or_default();
      *total = total.checked_add(amount).ok_or("total overflow")?;
      count += 1;
  }
  ```
  Match on the raw `fields[0]` (exact, case-sensitive). `map_or(true, …)` keeps the no-filter path identical and avoids an MSRV assumption about `Option::is_none_or`. Output formatting, `BTreeMap` ordering, and `minor_units` are untouched. The overflow check still applies to included rows only, matching the frozen baseline overflow test (which runs unfiltered).

### `examples/spendcheck/src/main.rs`
- Replace the `args.len() == 1` branch with a slice match over the collected args, dispatching into `report` / `report_filtered`:
  ```rust
  let result = match args.as_slice() {
      [path] => read_report(path, None),
      [path, flag, value] if flag == "--category" && !value.is_empty() =>
          read_report(path, Some(value)),
      _ => Err("usage: spendcheck <expenses.csv> [--category <name>]".into()),
  };
  ```
  `read_report` keeps the existing `read_to_string(...).map_err(...).and_then(|t| spendcheck::report_filtered(&t, category))`. The `Ok` → `print!`, `Err` → `eprintln!` + `exit(1)` tail is unchanged, preserving the single atomic stdout write.
- This grammar yields exactly the frozen rejections: `[path,"--category"]` (2 elements, no arm), `[path,"--unknown","travel"]` (guard false), `[path,"--category",""]` (empty value, guard false), and `[path,"--category","travel","extra"]` (4 elements) all fall to `_` → usage error, exit 1, empty stdout.

### `examples/spendcheck/README.md`
- Document the `--category <name>` flag: exact case-sensitive match; a valid category with no matches prints `records: 0` and exits 0; every row is validated before filtering so a malformed excluded row still fails. Keep the existing CSV-subset and currency/rounding description. Prose only.

**Unchanged:** `Cargo.toml` (std only, no new dependencies), `expenses.csv`, `tests/baseline.rs`, `tests/category.rs`.

## Failure handling

| Condition | Detected by | User sees |
|---|---|---|
| Malformed row (field count / currency / amount), even one excluded by filter | `report_filtered` validation loop, before filtering | stderr `invalid … at line N`, exit 1, empty stdout |
| Total overflow on an included row | `checked_add` in the loop | stderr `total overflow`, exit 1, empty stdout |
| Valid category, zero matches | empty `totals` map | `records: 0\n`, exit 0 |
| Missing value / empty name / unknown flag / trailing arg | `main` slice match `_` arm | stderr usage line, exit 1, empty stdout |
| Partial write | unreachable — report string fully built before the single `print!` | never partial |
| Retry / interruption / duplicate run | pure read→compute→print, no persisted state | idempotent, identical output each run |

The subtle invariant — validate-before-filter — is owned by keeping validation unconditional and gating only accumulation. The likely defect is moving the filter above validation so an excluded malformed row passes; the negative acceptance case below catches it.

## Executable acceptance

All commands execute real code (`cargo test` runs the suites; `cargo run` runs the built binary). None run in this plan phase.

Frozen suites (must stay green, unchanged):
```sh
cargo test --manifest-path examples/spendcheck/Cargo.toml --test baseline
cargo test --manifest-path examples/spendcheck/Cargo.toml --test category
```

Runtime checks:
```sh
# unfiltered unchanged -> records: 4\nEUR 14.50\nUSD 7.00\n , exit 0
cargo run --manifest-path examples/spendcheck/Cargo.toml -- examples/spendcheck/expenses.csv
# filtered, refund-aware -> records: 3\nEUR 10.00\nUSD 7.00\n , exit 0
cargo run --manifest-path examples/spendcheck/Cargo.toml -- examples/spendcheck/expenses.csv --category travel
# no matches: empty success -> records: 0\n , exit 0
cargo run --manifest-path examples/spendcheck/Cargo.toml -- examples/spendcheck/expenses.csv --category rent
# case-sensitive: no match -> records: 0\n , exit 0
cargo run --manifest-path examples/spendcheck/Cargo.toml -- examples/spendcheck/expenses.csv --category Travel
```

Negative cases (defect-catching):
```sh
# malformed EXCLUDED row still fails: stderr "line 3", exit 1, stdout empty
printf 'category,currency,amount\ntravel,EUR,1.00\nfood,EUR,bad\n' > /tmp/sc.csv
cargo run --manifest-path examples/spendcheck/Cargo.toml -- /tmp/sc.csv --category travel
# arg rejections: each exit 1, empty stdout
cargo run --manifest-path examples/spendcheck/Cargo.toml -- /tmp/sc.csv --category
cargo run --manifest-path examples/spendcheck/Cargo.toml -- /tmp/sc.csv --unknown travel
cargo run --manifest-path examples/spendcheck/Cargo.toml -- /tmp/sc.csv --category ""
cargo run --manifest-path examples/spendcheck/Cargo.toml -- /tmp/sc.csv --category travel extra
```

`cargo test` compiles **and executes** the suites; `cargo run` compiles **and executes** the binary. No step here is build-only.

## Prerequisites

- No new dependencies; no `Cargo.toml` change.
- Durable home when landed: `docs/plans/2026-10-25-spendcheck-category-filter.md`, indexed from the docs index.

---

Note: both the `Write` tool (plan file) and `ExitPlanMode` are disabled this session, so the plan is returned as prose above rather than written to `/Users/decebaldobrica/.claude/plans/…` or submitted for approval. No files edited, no tasks/runs created, nothing compiled or published. Frozen `tests/category.rs`, `tests/baseline.rs`, and `expenses.csv` untouched.
