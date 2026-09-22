#!/bin/bash
# r214: run a command; kill rust_verify/z3 if available memory drops below 4 GB.
"$@" &
pid=$!
while kill -0 $pid 2>/dev/null; do
    avail=$(awk '/MemAvailable/ {print int($2/1048576)}' /proc/meminfo)
    if [ "$avail" -lt 4 ]; then
        echo "MEMGUARD: available ${avail} GB < 4 GB, stopping run" >&2
        pkill -f z3; pkill -f rust_verify
        break
    fi
    sleep 2
done
wait $pid
