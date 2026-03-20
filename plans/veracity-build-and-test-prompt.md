# Veracity Agent: Build, Test, and Verify New Features

## Context

You committed changes for plan 1 (cfg hidden function detection) and possibly
plan 2 (root-cause vs downstream holes). But the binary that APAS-VERUS uses is
at `~/projects/veracity/target/release/veracity-review-proof-holes`. If you only
committed source changes without rebuilding, the new features won't be visible.

## Tasks

### 1. Build the release binary

```bash
cd ~/projects/veracity
cargo build --release
```

There is no debug build for veracity — always use `--release`.

Verify the binary is fresh:
```bash
ls -la target/release/veracity-review-proof-holes
```

### 2. Test against APAS-VERUS

Run the hole detector against the real codebase:
```bash
~/projects/veracity/target/release/veracity-review-proof-holes \
  -e benches -e tests -e rust_verify_test -e src/vstdplus \
  -e src/standards -e src/experiments ~/projects/APAS-VERUS
```

### 3. Verify plan 1: cfg hidden function detection

We stripped all cfg-gated functions from APAS-VERUS already, so there should be
zero findings. To test that the detection works, you could:
- Temporarily add a `#[cfg(not(verus_keep_ghost))]` to a function in a test file
- Run the detector and confirm it flags it
- Remove the test change

If you can't test with a real example, at least confirm the code path exists and
the output format is sensible.

### 4. Verify plan 2: root-cause vs downstream holes

Run against APAS-VERUS and check:

**Expected for Chap38 BSTParaMtEph.rs (13 holes):**
- 1 root cause: `expose_internal` (RwLock acquire_read — doesn't call other external_body fns)
- 12 downstream: all other external_body functions call `expose_internal`

**Expected for Chap39 BSTParaTreapMtEph.rs (16 holes):**
- 1 root cause: `expose_internal`
- ~14 downstream

The output should distinguish these. If automatic detection isn't implemented yet,
at minimum support the `// veracity: blocked_by(fn_name)` annotation from the plan.

### 5. Report

After building and testing, report:
- Which features are working
- Which features need more work
- Any bugs found during testing
- The output for Chap38 and Chap39 specifically (to verify root-cause detection)
