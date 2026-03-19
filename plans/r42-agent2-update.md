# R42 Agent 2 Update — Read and Continue

New mandatory rule:

**DO NOT move code outside `verus!{}` or add `#[cfg(not(verus_keep_ghost))]` to
dodge verification.** All algorithm implementations belong inside `verus!{}`.
Moving code out of Verus's scope to reduce the hole count is cheating — the code
is still unverified. If you can't prove it, leave the `external_body` and report
what you tried.

Commit what you have, push, and continue proving OrderedTableStPer methods.
