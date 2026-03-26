# Veracity: APAS-VERUS Validation Infrastructure

## validate.sh modes

```bash
scripts/validate.sh                    # full crate — all chapters, 5127 verified, ~8GB, ~100s
scripts/validate.sh isolate Chap55     # Chap55 + transitive deps only, ~1.5GB, ~18s
scripts/validate.sh isolate Chap65     # Chap65 + transitive deps only, ~2.7GB, ~78s
scripts/validate.sh isolate Chap43     # Chap43 + 8 transitive deps, ~2.3GB, ~29s
scripts/validate.sh full --profile     # full crate with Z3 quantifier profiling
scripts/validate.sh full --time        # full crate with per-function timing
```

## How isolate works

Each chapter has a Cargo feature in `Cargo.toml` with its dependency list:
```toml
Chap55 = ["Chap19", "Chap37", "Chap41"]
Chap43 = ["Chap18", "Chap19", "Chap37", "Chap38", "Chap41", "Chap42"]
Chap65 = ["Chap05", "Chap06", "Chap45"]
```

`validate.sh isolate ChapNN` reads the dep table, computes the transitive closure,
and passes `--cfg 'feature="isolate"'` + `--cfg 'feature="ChapNN"'` + all transitive
dep features to Verus.

In `src/lib.rs`, each chapter has:
```rust
#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap55")))]
pub mod Chap55 { ... }
```

Read as: include this chapter if we're NOT isolating, OR if this chapter is explicitly
included. Foundation modules (Types, Concurrency, vstdplus) have no isolate cfg — always
included.

## Memory constraints

- Machine: 32 GB RAM
- Full validate: ~8 GB rust_verify + up to 3 GB Z3 = ~11 GB peak
- Isolated validate: ~1.5-3 GB depending on chapter
- Two full validates concurrently WILL OOM (crashed Z3, killed processes)
- Three isolated validates can run concurrently (~9 GB total)

## validate.sh output

```
Starting verification at HH:MM:SS
Isolate: including Chap38 Chap37 Chap43 Chap42 Chap41 Chap18 Chap19 Chap23 Chap02
verification results:: 2517 verified, 0 errors
Elapsed: 29s
Sampled Memory Usage: peak rust_verify RSS: 2324MB, peak z3 RSS: 319MB, min free: 22512MB
Sampled CPU Usage: rust_verify: 41s, z3 children: 45s
```

The memory and CPU lines are sampled every 2s by a background monitor. Z3 CPU is
cumulative across all reaped children (from rust_verify's cutime+cstime).

## When to use what

| Task | Command | Why |
|------|---------|-----|
| Agent working on ChapNN | `validate.sh isolate ChapNN` | Low memory, fast iteration |
| Orchestrator after merge | `validate.sh` | Full crate verification |
| Debugging rlimit/trigger | `validate.sh full --profile` | Z3 quantifier profiling |
| Veracity tocify/style | `validate.sh` before + after | Must verify no regressions across full crate |
| Veracity hole analysis | `scripts/holes.sh src/ChapNN/` | Per-chapter, no compilation needed |
| Veracity all-holes | `scripts/all-holes-by-chap.sh` | All chapters, writes to analyses/ |

## RTT and PTT

```bash
scripts/rtt.sh              # cargo nextest — full compile + run tests (~2-3 GB)
scripts/rtt.sh Chap55       # filter to Chap55 tests only (still full compile)
scripts/ptt.sh              # full Verus compile of PTT crate + run proof tests (~8 GB)
```

RTT always compiles the full crate (cargo, not Verus). No isolation support.
PTT always does a full Verus compile. No isolation support.

Agents should NOT run RTT or PTT — orchestrator only.

## Logs

All scripts log to `logs/` with timestamped filenames:
```bash
ls -t logs/validate.*.log | head -1 | xargs cat   # last validate
ls -t logs/rtt.*.log | head -1 | xargs cat         # last RTT
ls -t logs/ptt.*.log | head -1 | xargs cat         # last PTT
```

Read logs instead of re-running to save time and memory.
