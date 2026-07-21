% CPR$ — Code, Proof, Review Costs
% Veracity Project
% 2026-05-09

# CPR$ — Three Numbers and One Ratio {.shrink}

| # | Symbol | Name          | Measures                                                |
|--:|:-------|:--------------|:--------------------------------------------------------|
| 1 | C      | Cost of Code  | $ to produce KLOE (code bodies, signatures, types)      |
| 2 | P      | Cost of Proof | $ to produce KLOP (specs, proof fns, proof-time tests)  |
| 3 | C+P    | Total         | Full bill for the verified artifact (cost of KLOC)      |
| 4 | R      | Review Ratio  | Fraction of proof a proofgrammer must review            |

- C and P are kept **separate** — they scale differently with project
  size, domain, and prover
- Most engineering decisions hinge on **which is bigger**

# Inputs & Computation {.shrink}

- **Tool inputs**: `veracity-count-loc` + `veracity-count-lines-of-review`
- **External inputs**: `--programmer-costs`, `--ai-costs`,
  `--person-days`, `--average-hours-per-day`
- **Fixed constants**: 220 days/yr × 8 hrs/day = **1,760 hrs/yr**
```
total_hours     = person_days × avg_hrs_per_day
programmer_cost = total_hours × (programmer_costs / 1,760)
AI_cost         = total_hours × (ai_costs        / 1,760)
KLOE = exec code bodies + signatures + type declarations
KLOP = specs + proof fns + proof-time tests
KLOC = KLOE + KLOP
C = total_cost × (KLOE / KLOC)
P = total_cost × (KLOP / KLOC)
R = LOPC2R / (LOPC2R + LOC0R)   # proof reviewed / all proof
```

# Worked Example — Inputs {.shrink}

| # | Project               | Hours    | KLOE   | KLOP   | KLOC    |
|--:|:----------------------|---------:|-------:|-------:|--------:|
| 1 | APAS-AI + Rusticate   |   309.6  | 31.751 |    —   |  31.751 |
| 2 | APAS-VERUS + Veracity | 1,123.4  | 65.481 | 70.650 | 136.131 |
| 3 | Combined              | 1,432.96 | 97.232 | 70.650 | 167.882 |

- **KLOE** = exec bodies 56,549 + signatures 6,972 + type decls 1,960 (Verus)
- **KLOP** = requires/ensures 20,298 + spec-fn bodies 7,723 + proof code
  25,627 + lemma bodies 6,009 + proof-time tests 10,993 (Verus)
- **Programmer rate**: $375,000/yr (senior + 50% loading)
- **Real AI spend**: ≤ $7,000 across the whole effort
- **AI share of total cost**: just **2.2%**
- **R inputs**: LOPC2R 47,828 / (47,828 + 95,908) = **33.3%**

# CPR$ — Combined Result (APAS-AI + APAS-VERUS) {.shrink}

| # | Quantity           | Value    | Definition                              |
|--:|:-------------------|---------:|:----------------------------------------|
| 1 | C — Cost of Code   | $185,252 | $ to produce KLOE (both projects)       |
| 2 | C / KLOE           | $1,905   | code $ per K exec line (KLOE 97.232)    |
| 3 | P — Cost of Proof  | $127,065 | $ to produce KLOP (Verus)               |
| 4 | P / KLOP           | $1,799   | proof $ per K proof line (KLOP 70.650)  |
| 5 | R — Review Ratio   | 33.3%    | proof reviewed / all proof              |
| 6 | C + P — Total      | $312,317 | full bill = cost of KLOC                 |
| 7 | (C + P) / KLOC     | $1,860   | $ per K code+proof line (KLOC 167.882)  |

- **C splits**: $67,477 (APAS-AI, all code) + $117,775 (APAS-VERUS code)
- The code rate ($1,905/K) exceeds the proof rate only because APAS-AI's
  unverified code cost more per line ($2,125/K) than APAS-VERUS ($1,799/K)

# CPR$ — APAS-VERUS Only {.shrink}

| # | Quantity           | Value    | Definition                              |
|--:|:-------------------|---------:|:----------------------------------------|
| 1 | C — Cost of Code   | $117,775 | $ to produce KLOE (Verus)               |
| 2 | C / KLOE           | $1,799   | code $ per K exec line (KLOE 65.481)    |
| 3 | P — Cost of Proof  | $127,065 | $ to produce KLOP                       |
| 4 | P / KLOP           | $1,799   | proof $ per K proof line (KLOP 70.650)  |
| 5 | R — Review Ratio   | 33.3%    | proof reviewed / all proof              |
| 6 | C + P — Total      | $244,840 | full bill = cost of KLOC                 |
| 7 | (C + P) / KLOC     | $1,799   | $ per K code+proof line (KLOC 136.131)  |

- APAS-VERUS bills at a **uniform ~$1,799/K** — code and proof lines cost
  the same, so C/KLOE, P/KLOP, and (C+P)/KLOC coincide
- Blank lines and comments are not billed separately; the full bill is
  attributed to code + proof

# Head-to-Head — seL4 (2009) vs APAS-VERUS — 22 py {.shrink}

| #  | Quantity         | seL4 (pre-AI) | APAS-VERUS (AI-paired) | Ratio          |
|---:|:-----------------|--------------:|-----------------------:|:---------------|
|  1 | Person-years     |           22  |                  0.64  | seL4 ~34×      |
|  2 | Hours            |       45,760  |                 1,123  | seL4 ~41×      |
|  3 | KLOE             |           10  |                    65  | Verus ~6.5×    |
|  4 | KLOP             |          480  |                    71  | seL4 ~6.8×     |
|  5 | KLOP / KLOE      |           48  |                   1.1  | seL4 **~44×**  |
|  6 | Klines / hour    |       0.0107  |                 0.121  | Verus ~11×     |
|  7 | $ / KLOE         |     $825,000  |               ~$3,739  | seL4 **~221×** |
|  8 | $ / KLOP         |      $17,188  |               ~$3,466  | seL4 ~5.0×     |
|  9 | C + P total      |  $8,250,000   |              $244,840  | seL4 **~33.7×**|

# Head-to-Head — seL4 (2009) vs APAS-VERUS — 22.5 py {.shrink}

| #  | Quantity         | seL4 (pre-AI) | APAS-VERUS (AI-paired) | Ratio          |
|---:|:-----------------|--------------:|-----------------------:|:---------------|
|  1 | Person-years     |         22.5  |                  0.64  | seL4 ~35×      |
|  2 | Hours            |       46,800  |                 1,123  | seL4 ~42×      |
|  3 | KLOE             |           10  |                    65  | Verus ~6.5×    |
|  4 | KLOP             |          480  |                    71  | seL4 ~6.8×     |
|  5 | KLOP / KLOE      |           48  |                   1.1  | seL4 **~44×**  |
|  6 | Klines / hour    |       0.0105  |                 0.121  | Verus ~12×     |
|  7 | $ / KLOE         |     $843,750  |               ~$3,739  | seL4 **~226×** |
|  8 | $ / KLOP         |      $17,578  |               ~$3,466  | seL4 ~5.1×     |
|  9 | C + P total      |  $8,437,500   |              $244,840  | seL4 **~34.5×**|

- **$ / KLOE** and **$ / KLOP** here divide the **whole bill** by each
  bucket (throughput / cost-per-line-produced) — distinct from the
  C / KLOE and P / KLOP cost-split in the result tables above
