use std::{collections::BTreeSet, fs, path::Path};

use allstack::infrastructure::digest;

fn collect(root: &Path, path: &Path, files: &mut BTreeSet<String>) {
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        let metadata = fs::symlink_metadata(&path).unwrap();
        assert!(!metadata.file_type().is_symlink());
        if metadata.is_dir() {
            collect(root, &path, files);
        } else {
            assert!(metadata.is_file());
            files.insert(path.strip_prefix(root).unwrap().to_str().unwrap().into());
        }
    }
}

#[test]
fn parquet_patch_preserves_published_source_except_compression_requirements() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("vendor/parquet");
    let metadata: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("upstream.json")).unwrap()).unwrap();
    assert_eq!(metadata["crate"], "parquet");
    assert_eq!(metadata["version"], "59.3.0");
    let expected = metadata["files"].as_object().unwrap();
    let mut actual = BTreeSet::new();
    collect(&root, &root, &mut actual);
    actual.remove("upstream.json");
    assert_eq!(actual, expected.keys().cloned().collect());
    for (name, checksum) in expected {
        let bytes = fs::read(root.join(name)).unwrap();
        let original = match name.as_str() {
            "Cargo.toml" => {
                let text = String::from_utf8(bytes).unwrap();
                assert_eq!(text.matches("version = \"0.14.3\"").count(), 2);
                text.replace("version = \"0.14.3\"", "version = \"0.13\"")
                    .into_bytes()
            }
            "Cargo.toml.orig" => {
                let text = String::from_utf8(bytes).unwrap();
                let updated = "zstd = { version = \"0.14.3\"";
                assert_eq!(text.matches(updated).count(), 2);
                text.replace(updated, "zstd = { version = \"0.13\"")
                    .into_bytes()
            }
            _ => bytes,
        };
        assert_eq!(digest(&original), checksum.as_str().unwrap(), "{name}");
    }
}
