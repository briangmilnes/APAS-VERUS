<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Proof Time Tests (PTTs)

PTTs verify that Verus specifications are satisfiable by writing small verified programs
that call library functions and prove properties about them. They run **verus** on
generated test files that import the compiled APAS-VERUS library.

## Architecture

```
rust_verify_test/
├── Cargo.toml          # Test binary definitions, depends on rust_verify_test_macros
├── tests/
│   ├── common/mod.rs   # Test harness: verify_one_file(), run_verus(), test macros
│   ├── Chap05/         # PTT files per chapter
│   ├── Chap17/
│   ├── Chap18/
│   └── Chap23/
```

Each PTT file uses the `test_verify_one_file!` macro (from `rust_verify_test_macros`)
to embed Verus code as a string, which the harness writes to a temp file and verifies
by invoking `verus` as a subprocess.

The harness (`common/mod.rs`) discovers the compiled library at:
- `target/verus/libapas_verus.rlib` — Rust compiled library
- `target/verus/apas_verus.vir` — Verus verification metadata (specs, proofs)

Both files must be present and **up to date** for PTTs to pass.

## Step 1: Compile the Library

This produces both the `.rlib` and `.vir` that PTTs import.

```bash
cd ~/projects/APAS-VERUS && ~/projects/verus/source/target-verus/release/verus \
  --compile \
  --crate-type=lib \
  --crate-name apas_verus \
  src/lib.rs \
  -o target/verus/libapas_verus.rlib \
  --export /home/milnes/projects/APAS-VERUS/target/verus/apas_verus.vir
```

**Flags explained:**

| Flag | Purpose |
|------|---------|
| `--compile` | Produce a compiled `.rlib` (not just verify) |
| `--crate-type=lib` | Build as library crate |
| `--crate-name apas_verus` | Set crate name (must match `use apas_verus::...` in PTTs) |
| `-o target/verus/libapas_verus.rlib` | Output path for compiled library |
| `--export .../apas_verus.vir` | **Export Verus metadata** — specs/proofs for cross-crate import |

**Critical:** The `--export` flag must use an **absolute path**. The `-o` flag uses a
relative path (relative to cwd). Without `--export`, only the `.rlib` is generated and
the `.vir` is NOT updated — PTTs will use stale specs and fail mysteriously.

## Step 2: Build the PTT Test Binaries

```bash
cd ~/projects/APAS-VERUS/rust_verify_test && cargo build --tests
```

This compiles the test harness and all PTT files defined in `rust_verify_test/Cargo.toml`.
The `rust_verify_test_macros` dependency requires nightly Rust.

## Step 3: Run PTTs

```bash
cd ~/projects/APAS-VERUS/rust_verify_test && cargo nextest run
```

Or to run all tests even if some fail:

```bash
cd ~/projects/APAS-VERUS/rust_verify_test && cargo nextest run --no-fail-fast
```

Or to run a specific test:

```bash
cd ~/projects/APAS-VERUS/rust_verify_test && cargo nextest run -E 'test(arrayseq_reduce)'
```

## Full Pipeline (All Three Steps)

```bash
# Step 1: Compile library with exported specs
cd ~/projects/APAS-VERUS && ~/projects/verus/source/target-verus/release/verus \
  --compile --crate-type=lib --crate-name apas_verus src/lib.rs \
  -o target/verus/libapas_verus.rlib \
  --export /home/milnes/projects/APAS-VERUS/target/verus/apas_verus.vir

# Step 2 + 3: Build and run PTTs
cd ~/projects/APAS-VERUS/rust_verify_test && cargo nextest run --no-fail-fast
```

## Troubleshooting

### "use of unresolved module or unlinked crate `apas_verus`"

The `.rlib` is missing or not at `target/verus/libapas_verus.rlib`. Re-run Step 1.

### "precondition not satisfied" pointing at source specs you've already changed

The `.vir` is stale. The `--export` flag was either missing or pointed to the wrong path.
Delete `target/verus/apas_verus.vir` and re-run Step 1 **with `--export`**.

### "could not create exported library file"

The `--export` path must be absolute. Relative paths fail because verus changes its
working directory internally.

### Tests pass locally but fail after merge

Another developer may have changed specs in the source. Re-run Step 1 to regenerate
both `.rlib` and `.vir` before running PTTs.

## Scripts

| # | Script | What it does |
|---|--------|-------------|
| 1 | `scripts/compile_ptt.py` | Compile library + export VIR (Step 1 only) |
| 2 | `scripts/pttest.py` | Full pipeline: compile + run all PTTs |
| 3 | `scripts/rttest.py` | Run runtime tests with ANSI stripping |
| 4 | `scripts/validate.py` | Full Verus verification |
| 5 | `scripts/dev_only_validate.py` | Dev-only Verus verification (foundation modules) |
| 6 | `scripts/exponlyverify.py` | Experiments-only Verus verification |

## Adding a New PTT

1. Create a test file at `rust_verify_test/tests/ChapNN/ProveModuleName.rs`
2. Add a `[[test]]` entry in `rust_verify_test/Cargo.toml`
3. Use the `test_verify_one_file!` macro pattern (see existing tests for examples)
4. The test code is embedded as a Verus code string using `verus_code_str!`
5. Import from `apas_verus::ChapNN::Module::Module::*` inside the embedded code
