fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let result = match args.as_slice() {
        [path] => read_report(path, None),
        [path, flag, value] if flag == "--category" && !value.is_empty() => {
            read_report(path, Some(value))
        }
        _ => Err("usage: spendcheck <expenses.csv> [--category <name>]".into()),
    };
    match result {
        Ok(report) => print!("{report}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn read_report(path: &str, category: Option<&str>) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .and_then(|text| spendcheck::report_filtered(&text, category))
}
