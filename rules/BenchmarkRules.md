## APAS Benchmark Rules

### Benchmark Timing Standards

#### Performance Targets (MANDATORY)
- **Individual benchmark time limit**: Each benchmark must complete in < 1.3 seconds
- **Warm-up time**: 300ms (0.3 seconds) per benchmark group
- **Measurement time**: 1 second per benchmark group
- **Sample size**: 30 samples per benchmark
- **Total run time per file**: ≤ 10 seconds
- **Configuration pattern**:
  ```rust
  fn bench_function(c: &mut Criterion) {
      let mut group = c.benchmark_group("GroupName");
      group.warm_up_time(Duration::from_millis(300));
      group.measurement_time(Duration::from_secs(1));
      group.sample_size(30);
      
      group.bench_function("test_name", |b| {
          b.iter(|| /* benchmark code */)
      });
      
      group.finish();
  }
  criterion_group!(benches, bench_function);
  criterion_main!(benches);
  ```

#### Rationale
- **Fast feedback loops**: Developers should get benchmark results quickly
- **CI/CD efficiency**: Total benchmark suite should complete in reasonable time
- **Sufficient statistical confidence**: 30 samples provides adequate confidence intervals
- **User time value**: Every minute of developer time waiting on slow benchmarks is expensive

### Unique Benchmark IDs (MANDATORY)

#### Rule: No Duplicate Benchmark IDs
- **Each benchmark within a group must have a unique ID**
- Criterion will panic at runtime if duplicate IDs are detected
- When using `BenchmarkId::new("name", param)`, the combination of name + param must be unique within the group

#### Common Mistakes to Avoid
```rust
// ❌ WRONG: Duplicate IDs in same loop iteration
for size in [100, 500, 1000].iter() {
    group.bench_with_input(BenchmarkId::new("filter", size), size, |b, _size| {
        b.iter(|| black_box(set.filter(|x| x % 2 == 0)));
    });
    
    // This creates duplicate ID "filter/100" when size=100!
    group.bench_with_input(BenchmarkId::new("filter", size), size, |b, _size| {
        b.iter(|| black_box(set.filter(|x| *x % 2 == 0)));
    });
}

// ✓ CORRECT: Different names for different operations
for size in [100, 500, 1000].iter() {
    group.bench_with_input(BenchmarkId::new("filter", size), size, |b, _size| {
        b.iter(|| black_box(set.filter(|x| x % 2 == 0)));
    });
    
    group.bench_with_input(BenchmarkId::new("map", size), size, |b, _size| {
        b.iter(|| black_box(set.map(|x| x * 2)));
    });
}
```

#### Validation
- Use `scripts/benches/check_duplicate_ids.py` to scan all benchmark files for duplicates
- The script checks that each `BenchmarkId::new(name, param)` combination is unique within its group
- Run before committing benchmark changes: `python3 scripts/benches/check_duplicate_ids.py`

#### Different Parameters Are OK
```rust
// ✓ CORRECT: Same name, different parameters = unique IDs
group.bench_function(BenchmarkId::new("med", "small_10"), |b| { ... });
group.bench_function(BenchmarkId::new("med", "medium_30"), |b| { ... });
// These create "med/small_10" and "med/medium_30" - both unique!
```

### File Organization and Naming

#### Directory Structure (MANDATORY)
- **One benchmark file per module**: Each source module gets exactly one benchmark file
- **Mirror source structure**: `benches/ChapXX/BenchModuleName.rs` mirrors `src/ChapXX/ModuleName.rs`
- **Chapter naming**: Benchmark files must include chapter suffix when benchmarking chapter-specific implementations
  - Pattern: `BenchModuleNameChapXX.rs` for modules that exist across multiple chapters
  - Example: `BenchArraySeqStEphChap18.rs` benchmarks `Chap18::ArraySeqStEph`
  - Example: `BenchArraySeqStEphChap19.rs` benchmarks `Chap19::ArraySeqStEph`
- **Module imports**: Benchmark files in `benches/ChapXX/` must import from `src/ChapXX/` modules
  ```rust
  use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::*;
  ```

#### Naming Conventions
- **Base pattern**: `BenchModuleName.rs` for unique modules
- **Chapter-specific pattern**: `BenchModuleNameChapXX.rs` for modules with multiple chapter implementations
- **Variant suffixes**: Include implementation variant in name
  - `BenchArraySeqStEph.rs` - Sequential/Ephemeral
  - `BenchArraySeqStPer.rs` - Sequential/Persistent  
  - `BenchArraySeqMtEph.rs` - Multithreaded/Ephemeral
  - `BenchArraySeqMtPer.rs` - Multithreaded/Persistent
- **Descriptive suffixes**: Add descriptive suffixes when one module has multiple benchmark files
  - `BenchAVLTreeSeqStEphChap37Ops.rs` - Operations benchmarks
  - `BenchAVLTreeSeqStEphChap37Build.rs` - Construction benchmarks

### Cargo.toml Registration (MANDATORY)

#### Registration Requirements
- **All benchmark files must be registered** in `Cargo.toml` under `[[bench]]` sections
- **Name must match filename** (without `.rs` extension)
- **Path must be correct** relative to project root
- **Example**:
  ```toml
  [[bench]]
  name = "BenchArraySeqStEphChap18"
  harness = false
  path = "benches/Chap18/BenchArraySeqStEphChap18.rs"
  ```

#### Validation
- Use `scripts/benches/check_cargo_bench_names.py` to verify all benchmark files are correctly registered
- Stale benchmarks (files not in Cargo.toml) should be deleted

### Benchmark Audit Script

#### Audit Tools
- **Location**: All benchmark audit scripts live in `scripts/benches/`
- **Primary script**: `scripts/benches/audit_one_benchmark.sh`
  - Audits a single benchmark file
  - Precompiles benchmarks for accurate timing
  - Reports compile time, run time, and total time
  - Validates each benchmark completes in < 1.3s
  - Dynamic timeout: `4 + (2 * num_benchmarks)` seconds
- **Batch script**: `scripts/benches/audit_first_n.sh`
  - Audits first N benchmark files
  - Calls `audit_one_benchmark.sh` for each file
- **Benchmark counter**: `scripts/benches/count_benchmark_runs.py`
  - Accurately counts individual benchmark runs in a file
  - Handles `for` loops with multiple input sizes
  - Used by audit script to set appropriate timeouts
- **Duplicate ID checker**: `scripts/benches/check_duplicate_ids.py`
  - Scans all benchmark files for duplicate `BenchmarkId::new(name, param)` combinations
  - Reports which functions have duplicate IDs
  - Exits with error code if duplicates found

#### Usage
```bash
# Audit a single benchmark file
scripts/benches/audit_one_benchmark.sh benches/Chap18/BenchArraySeqStEph.rs

# Audit first 50 files
scripts/benches/audit_first_n.sh 50

# Check for duplicate benchmark IDs
python3 scripts/benches/check_duplicate_ids.py

# Check Cargo.toml registration
python3 scripts/benches/check_cargo_bench_names.py
```

#### Audit Output Format
```
Auditing: BenchArraySeqStEph
Benchmarks: 15
  Compile: 2.3s
  Run: 21.4s
  Total: 23.7s
  ✓ All benchmarks < 1.3s
```

### Benchmark Configuration

#### Criterion Setup (MANDATORY)
- **Import pattern**:
  ```rust
  use criterion::{black_box, criterion_group, criterion_main, Criterion};
  use std::time::Duration;
  ```
- **Group configuration**: Every benchmark group must explicitly configure timing
  ```rust
  group.warm_up_time(Duration::from_millis(300));
  group.measurement_time(Duration::from_secs(1));
  group.sample_size(30);
  ```
- **Never rely on Criterion defaults**: Defaults are too slow (100 samples, longer measurement times)

#### Input Sizes
- **Representative sizes**: Use input sizes that represent realistic usage
- **Performance targets**: If a benchmark exceeds 1.3s, reduce input size
- **Document reductions**: When reducing input sizes for performance, add a comment explaining the original size and why it was reduced
  ```rust
  // Original size: 1000, reduced to 350 to meet 1.3s target
  let seq = ArraySeqMtPerS::tabulate(&|i| 350 - i, 350);
  ```

#### Benchmark Functions
- **Use `black_box`**: Prevent compiler optimizations from eliminating benchmark code
  ```rust
  group.bench_function("map", |b| {
      b.iter(|| ArraySeqStEphS::map(black_box(&seq), &increment))
  });
  ```
- **Setup outside `iter`**: Construct test data outside the `iter` closure
- **Minimal measurement scope**: Only measure the operation being benchmarked, not setup/teardown

### Macro Usage in Benchmarks

#### Construction Patterns
- **Persistent structs (`*Per`)**: Use `from_vec` pattern
  ```rust
  let data = vec![1, 2, 3, 4, 5];
  let seq = ArraySeqStPerS::from_vec(data);
  // Or let macro handle from_vec internally
  let seq = ArraySeqStPerSLit![1, 2, 3, 4, 5];
  ```
- **Ephemeral structs (`*Eph`)**: Use constructor + set pattern or tabulate
  ```rust
  let seq = ArraySeqStEphS::tabulate(&|i| i, 100);
  ```
- **Literal data**: Use `StructLit!` macros for compile-time known values
  ```rust
  let pairs = PairLit![(1, "alice"), (2, "bob")];
  ```

#### Critical Rule: Preserve Benchmarked Operations
- **NEVER replace the operation being benchmarked**
- Only use macros/helpers for test data setup, not the measurement target
- If benchmarking `tabulate`, `map`, `reduce`, etc., those exact calls must remain
  ```rust
  // CORRECT: Setup uses literal, benchmark measures tabulate
  group.bench_function("tabulate", |b| {
      b.iter(|| ArraySeqStEphS::tabulate(black_box(&identity), black_box(1000)))
  });
  
  // INCORRECT: Replacing tabulate with literal defeats the benchmark
  group.bench_function("tabulate", |b| {
      b.iter(|| ArraySeqStEphSLit![/* values */]) // This benchmarks the macro, not tabulate!
  });
  ```

### Exercise Benchmarks

#### Exercise Benchmark Policy
- **Default: Do not benchmark exercises**: Exercises are learning-focused implementations
- **Only create when requested**: Exercise benchmarks require explicit user approval
- **Naming convention**: `BenchExercise_X_Y.rs` where X is chapter and Y is exercise number
- **Configuration**: Follow same timing standards as regular benchmarks
- **Rationale**: Exercises demonstrate concepts; performance measurement is rarely needed

### Iterator Benchmarks

#### Representative Iterator Coverage
- **Provide iterator benchmarks**: Every sequence type should have iterator performance benchmarks
- **Pattern examples**:
  ```rust
  group.bench_function("iter_sum", |b| {
      b.iter(|| seq.iter().sum::<i32>())
  });
  
  group.bench_function("iter_collect", |b| {
      b.iter(|| seq.iter().cloned().collect::<Vec<_>>())
  });
  
  group.bench_function("iter_filter_map", |b| {
      b.iter(|| seq.iter().filter(|x| *x % 2 == 0).map(|x| x * 2).count())
  });
  ```
- **Rationale**: Iterator performance is critical for sequence types and must be measured

### Benchmark Quality Standards

#### Zero Warnings Policy
- **All benchmarks must compile with zero warnings**
- Apply same warning standards as source code
- Fix unused variables, imports, dead code immediately

#### Code Quality
- **Use named functions for closures**: When benchmarking trait methods that take `Fn` parameters, use named helper functions instead of inline closures
  ```rust
  // Helper functions at top of file
  fn identity(i: N) -> N { i }
  fn increment(x: &N) -> N { x + 1 }
  fn add(x: &N, y: &N) -> N { x + y }
  
  // In benchmark
  group.bench_function("map", |b| {
      b.iter(|| ArraySeqStEphS::map(black_box(&seq), &increment))
  });
  ```
- **Consistent formatting**: Follow project rustfmt configuration
- **Clear benchmark names**: Use descriptive names that indicate what is being measured

#### No Stale Benchmarks
- **Delete unregistered benchmarks**: If a benchmark file exists but is not in Cargo.toml, delete it
- **Delete duplicate benchmarks**: If multiple benchmarks cover the same functionality, keep the most comprehensive one
- **Regular audits**: Run benchmark audit script to identify and clean up stale benchmarks

### Multi-Input Benchmarks

#### Handling Multiple Input Sizes
- **Use loops for size variations**: When benchmarking across multiple input sizes
  ```rust
  for size in [10, 100, 1000].iter() {
      group.bench_function(&format!("tabulate_{}", size), |b| {
          b.iter(|| ArraySeqStEphS::tabulate(&identity, *size))
      });
  }
  ```
- **Timeout calculation**: Audit script multiplies base count by number of loop iterations
- **Keep within time budget**: Ensure all sizes combined still meet the 10s total file limit

### Cross-Chapter Benchmarks

#### When Modules Span Chapters
- **Benchmark both implementations**: If a module exists in both Chap18 and Chap19, create separate benchmark files
- **Import from correct chapter**: Each benchmark file imports from its chapter
  ```rust
  // benches/Chap18/BenchArraySeqStEphChap18.rs
  use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::*;
  
  // benches/Chap19/BenchArraySeqStEphChap19.rs
  use apas_ai::Chap19::ArraySeqStEph::ArraySeqStEph::*;
  ```
- **Compare performance**: Cross-chapter benchmarks allow comparing implementation strategies

### Summary Checklist

Before committing benchmark code, verify:
- [ ] Warm-up time set to 300ms
- [ ] Measurement time set to 1s  
- [ ] Sample size set to 30
- [ ] All individual benchmarks complete in < 1.3s
- [ ] No duplicate benchmark IDs (`python3 scripts/benches/check_duplicate_ids.py`)
- [ ] File registered in Cargo.toml with correct name and path
- [ ] File location matches source module structure
- [ ] Chapter suffix included when needed
- [ ] Imports from correct chapter
- [ ] Named functions used for closure parameters
- [ ] Black_box used to prevent optimization
- [ ] Setup code outside iter closure
- [ ] Zero compilation warnings
- [ ] Audit script passes for the file

