// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Standard 25: Algorithm Analysis Notation
//!
//! This standard defines the cost annotation format and variable naming
//! conventions used in `/// - Alg Analysis:` comments throughout APAS-VERUS.
//!
//! ## Annotation Format
//!
//! Every exec function that implements an APAS algorithm gets two lines:
//!
//! ```text
//! /// - Alg Analysis: APAS (ChNN Alg NN.N): Work O(...), Span O(...)
//! /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(...), Span O(...)
//! ```
//!
//! - The APAS line states the textbook's cost specification.
//! - The Code review line states the cost of our implementation.
//! - If they match, the Code review line may note this but does not need to.
//! - If they differ, the Code review line MUST say either:
//!   - `ACCEPTED DIFFERENCE: <reason>` — structural (Vec-backed, sequential, PRAM gap)
//!   - `DIFFERS: <explanation>` — unresolved, needs investigation
//! - Functions with no APAS cost spec get only the Code review line. Do NOT
//!   add an APAS line saying "not specified", "N/A", or "no cost stated".
//!
//! ## APAS Variable Naming
//!
//! APAS uses specific variable names for cost specifications. Our annotations
//! must use the SAME variables as the textbook for the APAS line. The Code
//! review line may use `n` as shorthand when unambiguous.
//!
//! | Variable    | Meaning                                    | Chapters       |
//! |-------------|--------------------------------------------|----------------|
//! | `n`         | Collection size (generic)                   | all            |
//! | `m`         | Second collection size (smaller of two)     | Chap37-43      |
//! | `|a|`, `|b|`| Sequence lengths                           | Chap18-19      |
//! | `|A|`       | Set size                                   | Chap41         |
//! | `|V|`, `|E|`| Vertex count, edge count                   | Chap52-66      |
//! | `|S|`, `|T|`| String lengths (edit distance)              | Chap49         |
//! | `u`         | Universe size (bit-vector sets)             | Chap41         |
//! | `h(T)`      | Tree height                                | Chap37         |
//! | `d`         | Graph diameter / BFS depth                 | Chap54         |
//! | `alpha`     | Load factor (hash tables)                  | Chap47         |
//! | `W(f)`, `S(f)` | Work/span of applied function f        | Chap18 map/filter/tabulate |
//!
//! ## BST Set Operation Bounds
//!
//! APAS gives tight merging bounds for balanced BST set operations:
//!
//! ```text
//! Work O(m log(n/m + 1)),  Span O(log n * log m)     where m <= n
//! ```
//!
//! - `m` is the size of the SMALLER set.
//! - `n` is the size of the LARGER set.
//! - This is tighter than the naive `O(n log n)` bound.
//! - When `m = 1` this gives O(log n) (single-element insert).
//! - When `m = n` this gives O(n) (equal-sized merge).
//!
//! Our Code review may state `O(n log n)` when the tight bound was not
//! analyzed. This is a valid ACCEPTED DIFFERENCE in analysis precision,
//! not an algorithm difference.
//!
//! ## Accepted Difference Categories
//!
//! | Category              | Example                                | Reason                          |
//! |-----------------------|----------------------------------------|---------------------------------|
//! | Vec-backed span       | Span O(n) instead of O(1) for subseq   | Vec copy vs tree slice          |
//! | St sequential span    | Span = Work for all St files            | No parallelism by design        |
//! | PRAM gap              | Span O(u) instead of O(1) for bit ops   | Fork-join cannot do PRAM O(1)   |
//! | Analysis precision    | O(n log n) instead of O(m log(n/m+1))   | Tight bound not analyzed        |
//! | Representation choice | O(lg n) instead of O(1) for nth         | Tree-backed instead of array    |
//! | Verus limitation      | Sequential filter instead of parallel    | spec_fn not Send                |
//!
//! ## Checking Annotations
//!
//! ```bash
//! scripts/check-alg-analysis.sh          # all levels, all chapters
//! scripts/check-alg-analysis.sh -e       # errors only (Emacs M-x compile)
//! scripts/check-alg-analysis.sh -w       # warnings only
//! scripts/check-alg-analysis.sh -e 41    # errors in Chap41
//! ```
//!
//! Output is in Emacs compilation-mode format (file:line: level: message).
