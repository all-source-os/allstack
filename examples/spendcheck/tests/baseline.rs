#[test]
fn totals_keep_currencies_separate_and_refunds_exact() {
    assert_eq!(
        spendcheck::report(include_str!("../expenses.csv")).unwrap(),
        "records: 4\nEUR 14.50\nUSD 7.00\n"
    );
}

#[test]
fn malformed_money_and_total_overflow_are_errors() {
    for rows in [
        "food,EUR,1.2",
        "food,EUR,NaN",
        "food,EUR,92233720368547758.07\nfood,EUR,0.01",
    ] {
        assert!(spendcheck::report(&format!("category,currency,amount\n{rows}\n")).is_err());
    }
}
