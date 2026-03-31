#!/bin/bash
# Acquire one of N exclusive slots before running a heavy Verus process.
# Source this file; it exports nothing but holds an flock on fd 9.
# The lock auto-releases when the calling script exits (fd 9 closes).
#
# Configuration (env vars, all optional):
#   VERUS_LOCK_DIR   — directory for lock files  (default: /tmp/verus-locks)
#   VERUS_LOCK_SLOTS — number of concurrent slots (default: 2)
#   VERUS_LOCK_TIMEOUT — seconds to wait before giving up (default: 300)
#
# Usage in validate.sh / ptt.sh:
#   source "$(dirname "${BASH_SOURCE[0]}")/verus-lock.sh"

VERUS_LOCK_DIR="${VERUS_LOCK_DIR:-/tmp/verus-locks}"
VERUS_LOCK_SLOTS="${VERUS_LOCK_SLOTS:-2}"
VERUS_LOCK_TIMEOUT="${VERUS_LOCK_TIMEOUT:-300}"

mkdir -p "$VERUS_LOCK_DIR"

_verus_lock_acquired=false
for (( _slot=1; _slot<=VERUS_LOCK_SLOTS; _slot++ )); do
    _lockfile="$VERUS_LOCK_DIR/verus-slot-${_slot}.lock"
    exec 9>"$_lockfile"
    if flock -n 9; then
        _verus_lock_acquired=true
        echo "Acquired verus lock slot ${_slot}/${VERUS_LOCK_SLOTS}"
        break
    fi
done

if [ "$_verus_lock_acquired" = false ]; then
    echo "All ${VERUS_LOCK_SLOTS} verus slots busy. Waiting up to ${VERUS_LOCK_TIMEOUT}s..."
    # Block on slot 1 with timeout.
    _lockfile="$VERUS_LOCK_DIR/verus-slot-1.lock"
    exec 9>"$_lockfile"
    if ! flock -w "$VERUS_LOCK_TIMEOUT" 9; then
        echo "ERROR: Could not acquire verus lock after ${VERUS_LOCK_TIMEOUT}s. Aborting."
        exit 1
    fi
    echo "Acquired verus lock slot 1/${VERUS_LOCK_SLOTS} (after waiting)"
fi

unset _verus_lock_acquired _slot _lockfile
