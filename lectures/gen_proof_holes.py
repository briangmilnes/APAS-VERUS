#!/usr/bin/env python3
"""Regenerate proof_holes.png from holes-over-time data."""

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker

# Data from lectures/quantitatives/apas-verus/holes-over-time.md
# plus R202 (2026-04-12, 0 holes) from veracity-review-verus-proof-holes
data = [
    ("R20",  20,  238),
    ("R30",  30,  189),
    ("R40",  40,  186),
    ("R50",  50,   34),
    ("R60",  60,   18),
    ("R70",  70,    0),
    ("R80",  80,    8),
    ("R90",  90,   41),
    ("R100", 100,  34),
    ("R110", 110,   7),
    ("R120", 120,  19),
    ("R130", 130,  30),
    ("R140", 140,   9),
    ("R150", 150,   6),
    ("R160", 160,   4),
    ("R170", 170,   4),
    ("R180", 180,   4),
    ("R190", 190,   4),
    ("R195", 195,   4),
    ("R196", 196,   4),
    ("R202", 202,   0),
]

labels  = [d[0] for d in data]
rounds  = [d[1] for d in data]
holes   = [d[2] for d in data]

fig, ax = plt.subplots(figsize=(11, 6))

ax.plot(rounds, holes, color='steelblue', linewidth=2.5, marker='o',
        markersize=5, zorder=3)

# Annotate start and end
ax.annotate('238', xy=(20, 238), xytext=(28, 230),
            fontsize=10, color='steelblue', fontweight='bold')
ax.annotate('0 ✓', xy=(202, 0), xytext=(192, 12),
            fontsize=11, color='green', fontweight='bold')

# Shade the "stuck at 4" plateau
ax.axhspan(-2, 6, xmin=(160-18)/(202-18), xmax=1.0,
           alpha=0.08, color='orange', label='Send/Sync plateau (awaiting Verus upstream)')

ax.set_xlabel('Round', fontsize=13)
ax.set_ylabel('Proof Holes', fontsize=13)
ax.set_title('APAS-VERUS: Proof Holes Over Time  (R20 → R202)', fontsize=14, fontweight='bold')

ax.set_xlim(15, 207)
ax.set_ylim(-5, 260)
ax.yaxis.set_major_locator(ticker.MultipleLocator(50))
ax.yaxis.set_minor_locator(ticker.MultipleLocator(10))
ax.grid(True, which='major', linestyle='--', alpha=0.4)
ax.grid(True, which='minor', linestyle=':', alpha=0.2)

# Mark the R70 first-clean and R202 final-clean
ax.axhline(0, color='green', linewidth=1.0, linestyle='--', alpha=0.5)

# X-tick every 20 rounds
tick_rounds  = [r for r in rounds if r % 20 == 0 or r == 195 or r == 202]
tick_labels  = [l for l, r, _ in data if r % 20 == 0 or r == 195 or r == 202]
ax.set_xticks(tick_rounds)
ax.set_xticklabels(tick_labels, fontsize=9, rotation=30)

ax.legend(fontsize=9, loc='upper right')
fig.tight_layout()
fig.savefig('proof_holes.png', dpi=150, bbox_inches='tight')
print("Written: proof_holes.png")
