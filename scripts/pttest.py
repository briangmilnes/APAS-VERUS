#!/usr/bin/env python3
"""Compile library then run proof time tests (PTTs) with cargo nextest."""

import subprocess
import sys
import re
import os

VERUS = "/home/milnes/projects/verus/source/target-verus/release/verus"
PROJECT = "/home/milnes/projects/APAS-VERUS"
PTT_DIR = os.path.join(PROJECT, "rust_verify_test")
VIR_PATH = os.path.join(PROJECT, "target", "verus", "apas_verus.vir")
RLIB_PATH = os.path.join(PROJECT, "target", "verus", "libapas_verus.rlib")


def strip_ansi_codes(text):
    """Strip ANSI escape codes for clean emacs compile mode output."""
    text = re.sub(r'\x1b\[[0-9;]*m', '', text)
    text = re.sub(r'\x1b\[[0-9]*[ABCDEFGHJKST]', '', text)
    return text


def main():
    os.makedirs(os.path.join(PROJECT, "target", "verus"), exist_ok=True)

    print("Compiling library for PTTs...", flush=True)
    print("=" * 60, flush=True)
    compile_result = subprocess.run(
        [VERUS, "--compile", "--crate-type=lib", "--crate-name", "apas_verus",
         "src/lib.rs", "-o", RLIB_PATH, "--export", VIR_PATH],
        cwd=PROJECT,
    )
    if compile_result.returncode != 0:
        print("Library compilation failed.", flush=True)
        return compile_result.returncode

    print("\nRunning PTTs with cargo nextest...", flush=True)
    print("=" * 60, flush=True)

    process = subprocess.Popen(
        ["cargo", "nextest", "run", "--no-fail-fast", "-j", "10"],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
        cwd=PTT_DIR,
    )

    for line in process.stdout:
        print(strip_ansi_codes(line), end='', flush=True)

    return process.wait()


if __name__ == "__main__":
    sys.exit(main())
