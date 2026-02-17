#!/usr/bin/env python3
"""Run full Verus verification on all modules."""

import subprocess
import sys

VERUS = "/home/milnes/projects/verus/source/target-verus/release/verus"
PROJECT = "/home/milnes/projects/APAS-VERUS"

result = subprocess.run(
    [VERUS, "--crate-type=lib", "src/lib.rs", "--multiple-errors", "20", "--expand-errors"],
    cwd=PROJECT,
)
sys.exit(result.returncode)
