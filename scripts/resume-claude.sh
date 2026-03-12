#!/usr/bin/env bash
# Resume a suspended (Ctrl+Z) Claude Code process without disturbing running agents.
# Finds Claude processes in stopped state (T) and sends SIGCONT.

found=0
for pid in $(pgrep -f 'claude'); do
    state=$(ps -o stat= -p "$pid" 2>/dev/null)
    if [[ "$state" == *T* ]]; then
        echo "Resuming stopped Claude process: pid=$pid state=$state"
        kill -CONT "$pid"
        found=1
    fi
done

if [[ "$found" -eq 0 ]]; then
    echo "No suspended Claude processes found."
fi
