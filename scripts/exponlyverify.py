#!/usr/bin/env python3
"""Run Verus verification with experiments_only feature flag."""

import subprocess
import sys

VERUS = "~/projects/verus/source/target-verus/release/verus"
CMD = f"{VERUS} --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors --cfg 'feature=\"experiments_only\"'"

result = subprocess.run(CMD, shell=True, cwd="/home/milnes/projects/APAS-VERUS")
sys.exit(result.returncode)
