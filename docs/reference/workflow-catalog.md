# Workflow catalog

The [repository catalog](../../workflows/catalog.toml) selects versioned workflow definitions. `allstack catalog` lists validated entries, and `allstack catalog <id>` returns the selected definition path, method paths, digests, supported hosts, and stage order. Discovery opens no mutable project store. Catalog files are limited to 64 KiB and 32 entries; definitions and methods are each limited to 64 KiB and must be contained regular files under `workflows/` without symlinks or multiple hard links.

| Workflow | Plan method | Build method | Complete outcome |
| --- | --- | --- | --- |
| [delivery](../../workflows/delivery.toml) | Shape an accepted change | Implement the complete request | Verified commit and open PR |
| [bugfix](../../workflows/bugfix.toml) | Diagnose an evidenced failed condition | Repair its cause and add regression coverage | Verified repair and open PR |

Start a selected workflow with `allstack start --workflow bugfix --request docs/requests/repair.md --policy workflows/verification.toml --host codex`. Add commit/publication flags only under trusted user authority. Follow the [CLI guide](cli.md) for handoff, recovery, verification, and publication. Planning cannot compile; provide retained reproduction output when the diagnosis requires execution evidence.

All entries use the supported plan → build → verify → ship lifecycle. Storage, host, handoff, process, and publication declarations must match the implemented contract. Unsupported declarations and unknown fields fail rather than implying support. Workflow-specific behavior lives in the plan/build method documents. The catalog does not supply permissions or an arbitrary graph engine.

AllSource freezes selection identity, definition path, and definition/method bytes. Accepted runs do not reselect from a changed catalog. Verification binds the selected definition and methods, while the full candidate snapshot still covers repository content. Chronis uses the existing run identity and epoch claim; task descriptions include workflow identity. Prime records include workflow identity and source links. Historical `delivery` graph nodes remain discoverable.

Run `allstack init --check`, then `allstack init` with an existing valid policy to add catalog resources to a previously initialized repository. Fresh Rust setup uses `--profile rust`. Existing delivery files stay byte-identical; missing catalog files, host skills, and indexed guidance are added. Conflicts block writes. Legacy repositories without a catalog expose only `delivery`, and old retained runs keep their original event serialization and default definition path.

The [catalog design](../plans/2026-10-24-workflow-catalog-design.md) defines compatibility and authenticated acceptance. The [bug-fix reproduction request](catalog-bugfix-request.md) supplies the concrete repair used to verify this catalog expansion.
