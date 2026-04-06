# Veracity minimize-lib: Add APAS isolate mode for per-chapter validation

## Context

You are working in `~/projects/veracity-agent1`. Do not touch `~/projects/veracity`.

## Problem

`veracity-minimize-lib` runs `cargo verus build` for every iteration — full crate
verification. On APAS-VERUS that's ~100 seconds per run. The tool iterates many
times (commenting out one function, verifying, restoring, repeat). For a chapter
with 20 proof functions, that's 20 × 100s = 33 minutes minimum. With retries and
binary search, it's often 60+ minutes.

APAS-VERUS has `scripts/validate.sh isolate ChapNN` which verifies only one chapter
plus its transitive dependencies. This takes 10-30 seconds instead of 100. Using
isolate mode would make minimize-lib 3-10x faster.

## What to change

### New CLI option

Add `-p APAS` or `--project APAS` flag (may already exist from count-loc). When
set, also accept `--chapter ChapNN` (e.g., `--chapter Chap37`).

Usage:
```bash
veracity-minimize-lib -c ~/projects/APAS-VERUS -l src/vstdplus/seq_set.rs \
    -p APAS --chapter Chap37
```

### Change to run_verus

When `-p APAS --chapter ChapNN` is set, instead of `cargo verus build`, run:

```bash
scripts/validate.sh isolate ChapNN
```

from the codebase directory. Parse the output for `verification results::` line
to determine success (same as current stderr parsing).

The key behavior:
- `scripts/validate.sh isolate ChapNN` verifies ChapNN + its transitive deps only.
- It reads the dep table from `Cargo.toml` and passes `--cfg` flags.
- Foundation modules (Types, Concurrency, vstdplus) are always included.
- Exit code 0 = success, non-zero = failure.
- The output contains `verification results:: N verified, M errors`.

### Determining the chapter

If `--chapter` is not specified but `-p APAS` is, try to infer the chapter from
the `-l` library path or the files being minimized:
- If the library file is `src/ChapNN/Foo.rs`, use `ChapNN`.
- If the library file is `src/vstdplus/foo.rs`, there's no single chapter — fall
  back to full validation (vstdplus is a dependency of everything).

### run_verus signature

Change `run_verus` to accept an optional chapter:

```rust
fn run_verus(codebase: &Path, isolate_chapter: Option<&str>) -> Result<(bool, String)> {
    if let Some(chapter) = isolate_chapter {
        let mut cmd = Command::new("bash");
        cmd.current_dir(codebase);
        cmd.args(["scripts/validate.sh", "isolate", chapter]);
        let output = cmd.output()?;
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let combined = format!("{}\n{}", stdout, stderr);
        let success = combined.contains("0 errors");
        return Ok((success, combined));
    }
    // ... existing full-crate logic
}
```

Note: `scripts/validate.sh` outputs to both stdout and stderr (via tee). Check
both for the `verification results::` line.

### Logging

Log which mode is being used:
```
Mode: APAS isolate Chap37 (estimated 10-30s per iteration)
```
vs
```
Mode: full crate verification (estimated 90-120s per iteration)
```

## Testing

Test with:
```bash
veracity-minimize-lib -c ~/projects/APAS-VERUS -l src/vstdplus/seq_set.rs \
    -p APAS --chapter Chap37 --dry-run
```

Verify it would call `scripts/validate.sh isolate Chap37` instead of
`cargo verus build`.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. A string-hacking
detector will flag and kill tools that corrupt source syntax.
