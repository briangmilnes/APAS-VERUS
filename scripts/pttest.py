#!/usr/bin/env python3
"""Run proof time tests (PTT) using cargo nextest from CWD."""

import subprocess
import sys
import re


def strip_ansi_codes(text):
    """Strip ANSI escape codes for clean emacs compile mode output."""
    text = re.sub(r'\x1b\[[0-9;]*m', '', text)
    text = re.sub(r'\x1b\[[0-9]*[ABCDEFGHJKST]', '', text)
    return text


def main():
    print("Running PTT (proof time tests) with cargo nextest...", flush=True)
    print("=" * 60, flush=True)
    
    process = subprocess.Popen(
        ["cargo", "nextest", "run", "--no-fail-fast", "-j", "10", "--features", "full_verify"],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1
    )
    
    for line in process.stdout:
        print(strip_ansi_codes(line), end='', flush=True)
    
    return process.wait()


if __name__ == "__main__":
    sys.exit(main())

