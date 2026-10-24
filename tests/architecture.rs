use std::{fs, path::Path};

#[test]
fn dependency_direction_and_module_sizes_are_enforced() {
    fn walk(path: &Path) {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path);
                continue;
            }
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let source = fs::read_to_string(&path).unwrap();
            assert!(
                source.lines().count() <= 500,
                "module exceeds 500 lines: {}",
                path.display()
            );
            assert!(
                !source.contains("#[cfg(test)]"),
                "inline test module: {}",
                path.display()
            );
            if path.ends_with("domain.rs") {
                for forbidden in [
                    "infrastructure",
                    "application",
                    "presentation",
                    "clap::",
                    "std::fs",
                ] {
                    assert!(!source.contains(forbidden), "domain imports {forbidden}");
                }
            }
            if path.components().any(|c| c.as_os_str() == "application") {
                for forbidden in [
                    "crate::infrastructure",
                    "crate::presentation",
                    "std::fs",
                    "std::process::Command",
                ] {
                    assert!(
                        !source.contains(forbidden),
                        "application crosses boundary: {forbidden}"
                    );
                }
            }
        }
    }
    walk(Path::new(env!("CARGO_MANIFEST_DIR")).join("src").as_path());
}

#[test]
fn empty_test_units_do_not_hide_positive_execution_or_count_as_tests() {
    let count = allstack::infrastructure::gates::executed_tests;
    assert_eq!(count("test result: ok. 0 passed; 0 failed"), 0);
    assert_eq!(
        count("test result: ok. 0 passed; 0 failed\ntest result: ok. 3 passed; 0 failed"),
        3
    );
    assert_eq!(count("Summary [0.2s] 5 tests run: 5 passed, 0 skipped"), 5);
    assert_eq!(
        count("No tests found\n1 passed marker without runner summary"),
        0
    );
}
