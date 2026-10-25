use std::collections::BTreeMap;

pub fn report(input: &str) -> Result<String, String> {
    report_filtered(input, None)
}

pub fn report_filtered(input: &str, category: Option<&str>) -> Result<String, String> {
    let mut lines = input.lines();
    if lines.next() != Some("category,currency,amount") {
        return Err("expected category,currency,amount header".into());
    }
    let mut totals: BTreeMap<&str, i64> = BTreeMap::new();
    let mut count = 0;
    for (index, line) in lines.enumerate() {
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != 3 || fields[0].trim().is_empty() {
            return Err(format!("invalid record at line {}", index + 2));
        }
        let currency = fields[1];
        if currency.len() != 3 || !currency.bytes().all(|b| b.is_ascii_uppercase()) {
            return Err(format!("invalid currency at line {}", index + 2));
        }
        let amount = minor_units(fields[2])
            .ok_or_else(|| format!("invalid amount at line {}", index + 2))?;
        if category.is_none() || category == Some(fields[0]) {
            let total = totals.entry(currency).or_default();
            *total = total.checked_add(amount).ok_or("total overflow")?;
            count += 1;
        }
    }
    let mut output = format!("records: {count}\n");
    for (currency, amount) in totals {
        let sign = if amount < 0 { "-" } else { "" };
        let amount = amount.unsigned_abs();
        output.push_str(&format!(
            "{currency} {sign}{}.{:02}\n",
            amount / 100,
            amount % 100
        ));
    }
    Ok(output)
}

fn minor_units(text: &str) -> Option<i64> {
    let (negative, text) = text.strip_prefix('-').map_or((false, text), |s| (true, s));
    let (whole, fraction) = text.split_once('.')?;
    if whole.is_empty()
        || !whole.bytes().all(|b| b.is_ascii_digit())
        || fraction.len() != 2
        || !fraction.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let amount = whole
        .parse::<i64>()
        .ok()?
        .checked_mul(100)?
        .checked_add(fraction.parse::<i64>().ok()?)?;
    if negative {
        amount.checked_neg()
    } else {
        Some(amount)
    }
}
