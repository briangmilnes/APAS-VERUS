# R77 Agent 1 — Register missing RTTs in Cargo.toml and enable disabled tests

## Objective

51 test files in `tests/Chap*/` are not registered in `Cargo.toml` as `[[test]]` entries.
Register them all and ensure they compile and pass. Also find and re-enable any
commented-out `#[test]` functions that should now work.

## Baseline

- 4869 verified, 0 errors, 0 warnings
- 2619 RTT passed, 157 PTT passed
- 193 `[[test]]` entries in Cargo.toml, 244 test files exist

## Task 1: Register missing test files

Find all test files missing from Cargo.toml:
```bash
comm -23 \
  <(ls tests/Chap*/Test*.rs | sed 's|tests/||;s|\.rs||' | sort) \
  <(grep -A1 '^\[\[test\]\]' Cargo.toml | grep 'name' | sed 's/.*= "//;s/"//' | sort)
```

For each missing file, add a `[[test]]` entry in Cargo.toml following the existing pattern:
```toml
[[test]]
name = "Chap06/TestDirGraphStEph"
path = "tests/Chap06/TestDirGraphStEph.rs"
```

Sort entries by chapter number, then filename, matching the existing order in Cargo.toml.

## Task 2: Compile and run

After registering all tests:
1. `cargo build --tests 2>&1 | head -50` — check for compile errors.
2. Fix any compile errors (missing imports, type changes from recent rewrites, etc.).
3. `scripts/rtt.sh` — run all tests, confirm they pass.

Common issues to expect:
- Tests written for old API (pre-BTreeSet rewrite) may need `use` updates.
- Tests for Mt types may need the new `obeys_feq_clone` requires on constructors —
  but RTT runs outside verus!, so requires are not checked. Should be fine.
- Some tests may be genuinely broken and need fixes.

## Task 3: Re-enable commented-out tests

Search for `// #[test]` in test files:
```bash
grep -rn "// #\[test\]" tests/
```

Known disabled tests:
- `tests/Chap37/TestAVLTreeSeq.rs` — 2 tests marked "failing atm"
- `tests/Chap41/TestAVLTreeSetMtPer.rs` — ~10 tests disabled
- `tests/Chap06/TestLabUnDirGraphStEph.rs` — 1 test disabled

For each: uncomment `#[test]`, try to compile and run. If it passes, keep it. If it
fails with a real bug, leave it commented with a clearer note about what fails.

## Important

- Do NOT modify source files in `src/`. This is a test-only task.
- Do NOT modify `scripts/validate.sh` or other scripts.
- Register ALL 51 missing files, even if some fail to compile — comment out the failing
  ones in Cargo.toml with a reason, rather than omitting them silently.
- Run `scripts/rtt.sh` (not `cargo test`) to verify.

## Validation

Run `scripts/rtt.sh`. Push to `agent1/ready`.

## Report

Write `plans/agent1-round77-report.md` with:
- Number of tests registered
- Number of tests that compile and pass
- Number of tests that needed fixes (and what fixes)
- Number of tests that remain broken (and why)
- Number of commented-out tests re-enabled
