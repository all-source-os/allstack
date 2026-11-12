---
name: allstack-ui-handoff
description: Hand off observed interface defects to another delivery chat with reproducible scenarios, evidence, ownership and verification boundaries.
---

# Hand off interface fixes

Use when the user asks to delegate UI defects while review continues elsewhere. Establish the receiving chat's ownership and preserve existing work; do not assume a review checkout is disposable or that another chat may rewrite its history. Create or message a chat only with the user's authorization.

Give the receiving chat a self-contained brief containing:

- Product, environment, reachable URL or local launch steps, tested revision and date; distinguish deployed, staged and working-tree behavior.
- For each defect, starting state, viewport and input mode, exact actions and test data, observed result, expected result, user impact and evidence paths. Include all affected routes or a complete inventory when applicable.
- Relevant design decisions, accessibility and privacy constraints, existing checks and noindex requirements. Name source files and candidate changes as leads, not proof of a cause.
- Ownership of files or worktree, other active owners, uncommitted changes, permitted destinations and actions, plus explicit boundaries on deployments and unrelated work.
- Acceptance scenarios that repeat the actual journey, regression checks for shared consumers, screenshots tied to the tested source, and an honest list of unverified states. Require actual command and browser results, not a status claim alone.

Identify findings that remain hypotheses and missing reproduction prerequisites. Do not copy secrets or private customer data. Link the durable review record and provide enough context to work without the original transcript. Continue independent review without editing the recipient's files; send newly discovered defects only when messaging is authorized. Report the new chat and scope to the user.
