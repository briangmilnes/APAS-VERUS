#!/usr/bin/env python3
"""Compile APAS-VERUS library for proof time tests (PTTs).

Produces both:
  target/verus/libapas_verus.rlib  — compiled Rust library
  target/verus/apas_verus.vir     — Verus spec metadata for cross-crate import

Both must be fresh for PTTs to pass.
"""

import subprocess
import sys
import os

VERUS = "/home/milnes/projects/verus/source/target-verus/release/verus"
PROJECT = "/home/milnes/projects/APAS-VERUS"
VIR_PATH = os.path.join(PROJECT, "target", "verus", "apas_verus.vir")
RLIB_PATH = os.path.join(PROJECT, "target", "verus", "libapas_verus.rlib")

os.makedirs(os.path.join(PROJECT, "target", "verus"), exist_ok=True)

result = subprocess.run(
    [VERUS, "--compile", "--crate-type=lib", "--crate-name", "apas_verus",
     "src/lib.rs", "-o", RLIB_PATH, "--export", VIR_PATH],
    cwd=PROJECT,
)
sys.exit(result.returncode)
