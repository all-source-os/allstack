pub fn executed_tests(output: &str) -> usize {
    output
        .lines()
        .map(|line| {
            if let Some(summary) = line.trim().strip_prefix("test result: ok.") {
                return summary
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(0);
            }
            let words: Vec<_> = line.split_whitespace().collect();
            words
                .windows(3)
                .find_map(|w| {
                    (w[1] == "tests" && w[2] == "run:")
                        .then(|| w[0].parse::<usize>().ok())
                        .flatten()
                })
                .unwrap_or(0)
        })
        .sum()
}
