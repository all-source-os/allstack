# Fixed compression dependencies with Arrow 59 compatibility

Allstack's strict dependency audit rejects zstd 0.13.3, zstd-safe 7.3.0 and zstd-sys 2.1.0. The affected packages arrive through published `allsource-core 0.26.1` → `parquet 59.3.0`. The required fixed versions are zstd 0.14.3, zstd-safe 8.2.0 and zstd-sys 2.1.2.

Published compatible Parquet 59.3.1 and 59.3.2 still require zstd 0.13. The Arrow 59 maintenance branch has the same dependency constraint. Moving Parquet to a newer Arrow major would change the RecordBatch type boundary used by Core. Disabling compression would remove support for existing encoded data. Neither is necessary for this repair.

The root manifest patches Parquet to `vendor/parquet`, copied from the exact 59.3.0 crate archive whose checksum is recorded in the lockfile. Only its normalized and original manifests change: both runtime and development zstd requirements become 0.14.3. Library source, bundled Apache license and notice, codec features and version stay byte-identical to the published package. The lockfile selects the fixed compression stack. No advisory is ignored and no audit severity is reduced.

The patch keeps the existing Arrow/Parquet 59 API and compression formats. Qualification must compare the vendored files with the verified archive, execute strict dependency auditing, rebuild the installed toolkit, and exercise persistence reopening plus native delivery. Remove the local patch when a compatible upstream Parquet release carries the fixed dependency requirement; verify the actual manifest rather than assuming a patch-version bump includes it.

`tests/vendor_integrity.rs` verifies the complete file set and every upstream digest, reversing only the two manifest requirement edits before comparison. First-party formatting targets the Allstack package explicitly; external source stays byte-identical to its archive and is checked by the provenance test. This preserves first-party formatting coverage without rewriting upstream code.

The published `parquet-fromcsv-help.txt` fixture contains eight space-only help-output lines. Its file-specific Git whitespace attribute permits those trailing spaces; the integrity test still rejects any byte change to the fixture. Whitespace checks remain active for first-party source and all other files.
