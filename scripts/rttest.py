#!/usr/bin/env python3
"""Run runtime tests (RTTs) with cargo nextest."""

import subprocess
import sys
import re

PROJECT = "/home/milnes/projects/APAS-VERUS"


def strip_ansi_codes(text):
    """Strip ANSI escape codes for clean emacs compile mode output."""
    text = re.sub(r'\x1b\[[0-9;]*m', '', text)
    text = re.sub(r'\x1b\[[0-9]*[ABCDEFGHJKST]', '', text)
    return text


def main():
    print("Running RTTs with cargo nextest...", flush=True)
    print("=" * 60, flush=True)

    process = subprocess.Popen(
        ["timeout", "120", "cargo", "nextest", "run", "--no-fail-fast"],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
        cwd=PROJECT,
    )

    for line in process.stdout:
        print(strip_ansi_codes(line), end='', flush=True)

    return process.wait()


if __name__ == "__main__":
    sys.exit(main())
