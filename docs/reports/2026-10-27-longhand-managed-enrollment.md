# Longhand managed staff compatibility

## Scope

Longhand enrollment uses its complete tracked checkout and existing project Chronis history. Default snapshot bounds reject the assessed 11,246-file, 262,502,948-byte source tree. The repository can now declare a bounded snapshot budget in tracked workflows/snapshot-limits.toml; default bounds and per-file checks remain unchanged. Content-addressed artifacts retain every source file. No sparse checkout, excluded source subtree or independent tracker is substituted.

Linked worktrees resolve their existing project tracker from Git’s common directory. Initialization, doctor, task setup, claim transfer and completion use that owning project through cn. Run evidence remains private to the coordinator.

## Qualification

Thirteen bootstrap tests and six process-owned workspace cases passed; two optional real-store tests were skipped. The snapshot proof retained 10,013 entries above the original byte limit, restored complete source, reused primary tracker routing despite local setup metadata, and rejected independent worktree data. Actual Longhand init and doctor then passed with the primary project tracker readable and no duplicate tracker initialized. Real host execution remains separate. Process-owned regression cases exercise a checkout above both original bounds, restore added/edited/deleted content, preserve guarded base files and resolve the primary tracker without creating a worktree tracker. Invalid budgets remain rejected. No production or customer outcome follows from these toolkit checks.
