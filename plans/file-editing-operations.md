| # | Operation | Tool | Pros | Cons |
|---|-----------|------|------|------|
| 1 | **Exact String Replacement** | `replace_string_in_file` | Very safe; fails if the exact string (including whitespace) isn't found. Good for targeted edits. | Fails easily if the file changed slightly or if I miscalculate indentation. Requires providing 3-5 lines of context. |
| 2 | **Full File Rewrite** | `create_file` (overwrite) | Guarantees the file matches my exact mental model. Good for small files or complete rewrites. | Dangerous for large files; easy to accidentally delete user changes or introduce subtle regressions. |
| 3 | **Notebook Cell Edit** | `edit_notebook_file` | Safely targets specific cells in Jupyter notebooks without touching the rest of the file. | Only works on `.ipynb` files. |
| 4 | **Terminal `sed` / `awk`** | `run_in_terminal` | Powerful for bulk operations (e.g., deleting a range of lines, regex replacements). | Extremely brittle and dangerous. Easy to accidentally delete the wrong lines or break syntax (as I just demonstrated). |
| 5 | **Terminal `echo` / `cat`** | `run_in_terminal` | Good for appending to files (`>>`) or creating small new files (`>`). | Not suitable for complex edits in the middle of a file. |
