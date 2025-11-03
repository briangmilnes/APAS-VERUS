## Codex Execution Rules — Emacs Interaction

#### emacs_sync
- Trigger: when you say you’ll “fix/edit in Emacs,” pause all write edits immediately.
- Pause behavior: read-only work only (reviews/plans), no file edits until “Emacs: done”.
- Resync on done: propose refreshing repo state, then re-open changed files.
  - Suggested:
    ```bash
    git status -s | cat
    git diff --name-only | cat
    ```
- Reconcile: prefer your Emacs changes; never overwrite. If conflicts arise, summarize diffs and ask how to proceed.
- Style guarantees: keep existing indentation, whitespace, and CamelCase module naming intact.
- Scope hygiene: avoid global refactors while you’re editing; target only files you didn’t modify unless you approve.
- Follow-ups: after resync, run lint/diagnostics on changed files and resume the pending task.

#### Timestamping commands
- Prefix every command with the current timestamp via `date +"%d %H:%M:%S"` so the log shows when it ran.

#### Cargo command output
- After running any `cargo` command, display its output inline in the chat response.
- Present the complete command output inside a fenced code block so the full log stays visible (no truncation or collapsing).
- Include the exact `cargo` command line above the fenced output block so the command itself is visible.
- Before each tool call (plan updates, shell commands, etc.), state in chat which tool is being used and its purpose so the trace remains clear.

#### Scratch files and editor backups
- Treat any `.save*` files (and similar scratch/backup artifacts) as off-limits for reading—they may contain incomplete or transient edits.
- If such a file becomes relevant, wait for the user to provide its contents instead of opening it directly.
