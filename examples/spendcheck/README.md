# Spendcheck

Summarize expense records without combining currencies or rounding floating-point money. Input uses a strict three-column CSV subset: `category,currency,amount`, unquoted fields, uppercase three-letter currencies, and signed decimal amounts with exactly two fractional digits. Commas inside fields are unsupported.

```sh
cargo run --manifest-path examples/spendcheck/Cargo.toml -- examples/spendcheck/expenses.csv
```

The report prints the record count followed by totals sorted by currency. Invalid records and overflowing totals fail with a nonzero exit and no partial report.

Add `--category <name>` after the CSV path to report only records whose category matches exactly, including case:

```sh
cargo run --manifest-path examples/spendcheck/Cargo.toml -- examples/spendcheck/expenses.csv --category travel
```

Only matching records contribute to the count and per-currency totals, including refunds. A category with no matches prints `records: 0` and exits successfully. Every input row is validated before filtering, so a malformed excluded record still fails with its source line and no partial report. Missing values, empty category names, unknown flags, and trailing arguments are rejected.
