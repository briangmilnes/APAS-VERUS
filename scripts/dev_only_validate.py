#!/usr/bin/env python3
"""Run Verus verification with dev_only feature — foundation modules only.

To include specific chapters in dev_only mode, edit src/lib.rs and change their
#[cfg] from  not(any("experiments_only", "dev_only"))
to just      not(feature = "experiments_only")
"""

import subprocess
import sys

VERUS = "/home/milnes/projects/verus/source/target-verus/release/verus"
PROJECT = "/home/milnes/projects/APAS-VERUS"

result = subprocess.run(
    [VERUS, "--crate-type=lib", "src/lib.rs", "--multiple-errors", "20", "--expand-errors",
     "--cfg", 'feature="dev_only"'],
    cwd=PROJECT,
)
sys.exit(result.returncode)
