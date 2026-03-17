R33: BellmanFord/Johnson enum refactor + graph algorithm warnings.

TASK 1 — Replace String error type with enum.

The BellmanFord and Johnson algorithms use `Result<..., String>` with
external_body helper functions that construct error strings. Replace
with an enum to eliminate the external_body holes.

(a) Define the error enum in BellmanFordStEphI64.rs:
    ```
    pub enum BellmanFordError {
        NegativeCycleDetected,
        AlgorithmError,
    }
    ```
    Put it inside verus! in the type definitions section.

(b) src/Chap58/BellmanFordStEphI64.rs:
    - Delete `neg_cycle_error_string` (line ~85) and
      `algorithm_error_string` (line ~90) — both external_body.
    - Change `bellman_ford()` return type from
      `Result<SSSPResultStEphI64, String>` to
      `Result<SSSPResultStEphI64, BellmanFordError>`.
    - Replace `Err(neg_cycle_error_string())` with
      `Err(BellmanFordError::NegativeCycleDetected)`.
    - Replace `Err(algorithm_error_string())` with
      `Err(BellmanFordError::AlgorithmError)`.

(c) src/Chap59/JohnsonStEphI64.rs:
    - Import BellmanFordError from Chap58.
    - Delete `neg_cycle_error_string` (line ~69).
    - Update any pattern matches on bellman_ford results.
    - Johnson discards the error (`Err(_)`), so the type
      change should be transparent.

(d) Update tests:
    - tests/Chap58/TestBellmanFordStEphI64.rs:
      Replace `.err().unwrap().contains("Negative")` with
      pattern match `Err(BellmanFordError::NegativeCycleDetected)`.
    - tests/Chap58/TestBellmanFordStEphF64.rs: same pattern.
    - tests/Chap59/TestJohnsonStEphI64.rs: check if it tests errors.

TASK 2 — Fix remaining warnings in graph algorithm files.

(a) src/Chap57/DijkstraStEphI64.rs — fn_missing_requires (1).
    Read the function, determine real precondition.

(b) src/Chap58/BellmanFordStEphI64.rs — fn_missing_requires (1).
    Read the function, determine real precondition.

(c) src/Chap59/JohnsonStEphI64.rs — 3 requires_true warnings
    (adjust_distance, reweight_edge, create_negative_cycle_result).
    If genuinely no precondition, add `// veracity: no_requires`.
    If there IS a real precondition, add it.

Do NOT add assume, accept, or external_body.
Do NOT add `requires true` or tautological requires.
Every quantifier must have explicit #[trigger].
Run scripts/validate.sh after changes — 0 errors required.
