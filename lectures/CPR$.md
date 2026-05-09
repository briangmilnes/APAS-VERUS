% CPR$ — Code, Proof, Review Costs
% Veracity Project
% 2026-05-09

# CPR$ — Three Numbers and One Ratio {.shrink}

| # | Symbol | Name          | Measures                                                           |
|--:|:-------|:--------------|:-------------------------------------------------------------------|
| 1 | C      | Cost of Code  | $ to produce executable code                                       |
| 2 | P      | Cost of Proof | $ to produce specs, contracts, and proofs                          |
| 3 | C+P    | Total         | Full bill for the verified artifact                                |
| 4 | R      | Review Ratio  | Fraction of deliverable a proofgrammer must read (LOC0R excluded)  |

- C and P are kept **separate** — they scale differently with project
  size, domain, and prover
- Most engineering decisions hinge on **which is bigger**

# Inputs & Computation {.shrink}

- **Tool inputs**: `veracity-count-loc` + `veracity-count-lines-of-review`
- **External inputs**: `--programmer-costs`, `--ai-costs`,
  `--person-days`, `--average-hours-per-day`
- **Fixed constants**: 220 days/yr × 8 hrs/day = **1,760 hrs/yr**
```
total_hours      = person_days × avg_hrs_per_day
programmer_cost  = total_hours × (programmer_costs / 1,760)
AI_cost          = total_hours × (ai_costs        / 1,760)
LoEC_share       = LoEC / total_LOC
C = total_cost × LoEC_share
P = total_cost × (1 − LoEC_share)
R = LOPC2R / (LOPC2R + LOC0R)   # on the deliverable only```

# Worked Example — Inputs {.shrink}

| # | Project               | Hours    | LOC     | LoEC   | LOPC2R | LOC0R  |
|--:|:----------------------|---------:|--------:|-------:|-------:|-------:|
| 1 | APAS-AI + Rusticate   |   309.6  |  31,751 | 31,751 |      0 |      0 |
| 2 | APAS-VERUS + Veracity | 1,123.4  | 166,401 | 56,549 | 47,828 | 95,908 |
| 3 | Combined              | 1,432.96 | 198,152 | 88,300 | 47,828 | 95,908 |

- **Programmer rate**: $375,000/yr (senior + 50% loading)
- **Real AI spend**: ≤ $7,000 across the whole effort
- **Derived `--ai-costs`**: $4.886/hr × 1,760 = **$8,599/yr**
- **AI split by task-hours**: 21.6% APAS-AI ($1,512) / 78.4% APAS-VERUS ($5,488)

# CPR$ — Combined Result {.shrink}

| # | Quantity                         | Value         |
|--:|:---------------------------------|--------------:|
| 1 | C — Cost of Code                 | **$150,723**  |
| 2 | P — Cost of Proof                | **$161,594**  |
| 3 | C + P — Total                    | **$312,317**  |
| 4 | R — Review Ratio                 | **33.3%**     |
| 5 | $ / verified deliverable line    | $1.877        |
| 6 | C / LoEC                         | $2.665        |
| 7 | P / LOPC2R                       | $3.378        |
| 8 | P / LOP                          | $6.305        |

- **LoEC share** of APAS-VERUS = 56,549 / 166,401 = **34.0%**
- C splits as $67,477 (APAS-AI) + $83,246 (APAS-VERUS code portion)
- P comes entirely from the APAS-VERUS spec/proof portion
- **AI share of total cost**: just **2.2%**

# Head-to-Head — seL4 (2009) vs APAS-VERUS {.shrink}

| #  | Quantity         | seL4 (pre-AI) | APAS-VERUS (AI-paired) | Ratio              |
|---:|:-----------------|--------------:|-----------------------:|:-------------------|
|  1 | Person-years     |           22  |                  0.64  | seL4 ~34×          |
|  2 | Hours            |       45,760  |                 1,123  | seL4 ~41×          |
|  3 | KLOE             |           10  |                  ~57   | Verus ~5.7×        |
|  4 | KLOP             |          480  |                  ~110  | seL4 ~4.4×         |
|  5 | KLOP / KLOE      |           48  |                  ~1.9  | seL4 **~25×**      |
|  6 | Klines / hour    |       0.0107  |                 0.148  | Verus ~14×         |
|  7 | $ / KLOE         |     $825,000  |               ~$4,300  | seL4 **~192×**     |
|  8 | $ / KLOP         |      $17,188  |               ~$2,225  | seL4 ~7.7×         |
|  9 | C + P total      |  $8,250,000   |              $244,840  | seL4 **~33.7×**    |

