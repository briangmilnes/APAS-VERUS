# R160 Prompt D — Minimize Chap55 DFS Proofs

Agent: agent4 | Round: R160

## Results

| # | Chap | File | Asserts Before | Asserts After | Delta |
|---|------|------|---------------|--------------|-------|
| 1 | 55 | CycleDetectStEph.rs | 182 | 157 | -25 |
| 2 | 55 | CycleDetectStPer.rs | 143 | 123 | -20 |
| 3 | 55 | TopoSortStEph.rs | 177 | 146 | -31 |
| 4 | 55 | TopoSortStPer.rs | 118 | 97 | -21 |
| — | — | **Total** | **620** | **523** | **-97** |

## Validation

- Full `scripts/validate.sh`: 5748 verified, 0 errors
- Isolate `scripts/validate.sh isolate Chap55`: 2396 verified, 0 errors

## Techniques Used

**Always removable:**
- `ok.is_ok()` after `set()` — success guaranteed when bounds met
- `spec_len == @.len()` after bridge lemma
- `@.len() == graph@.len()` when wf + bridge already establish this
- `spec_num_false < spec_num_false(old(...))` after the two lemma calls
- Neighbor chain: `assert(neighbor == graph@[v][i])`, `graph@[v][i] < graph@.len()`, `neighbor < graph@.len()` — derivable from loop invariant + wf
- Monotonicity `assert forall ... old(visited)@[j] implies visited@[j]` — derivable from `visited@ =~= old(visited)@.update(...)`

**Kept (proof-critical):**
- `assert(!old(visited)@[vertex as int])` — bridges if-condition to `@` view for Z3
- `assert(vertex < visited.spec_len())` and `assert(vertex < rec_stack.spec_len())` — needed for `set()` requires; removing causes Z3 instability (Z3 CPU 63s → 1057s)
- `assert(vertex < graph.spec_len())` — needed for `graph.nth(vertex)` and bridge
- `assert(*neighbors == graph.spec_index(vertex as int))` — needed for bridge lemma trigger
- `assert(visited@ =~= old(visited)@.update(...))` — StEph requires explicit extensional equality
- Bridge lemma calls — mandatory for StEph `@` view access

## Stability Notes

- Removing `vertex < visited.spec_len()` + `vertex < rec_stack.spec_len()` together
  from `dfs_finish_order_cycle_detect` (TopoSortStEph) caused Z3 CPU to spike from 63s
  to 1057s (proof instability, not failure). Restored both.
- RAM monitored throughout; stable at 2.2GB isolate, 11.7GB full.
