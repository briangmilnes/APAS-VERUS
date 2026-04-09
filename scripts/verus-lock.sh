#!/bin/bash
# Acquire exclusive slots before running a heavy Verus process.
# Source this file; it holds flocks on fd 8 and/or 9.
# Locks auto-release when the calling script exits (fds close).
#
# Configuration (env vars, all optional):
#   VERUS_LOCK_DIR     — directory for lock files  (default: /tmp/verus-locks)
#   VERUS_LOCK_SLOTS   — number of concurrent slots (default: 3)
#   VERUS_LOCK_TIMEOUT — seconds to wait before giving up (default: 300)
#   VERUS_LOCK_WEIGHT  — slots to acquire: 1 for isolate, 2 for full (default: 1)
#
# Weight policy:
#   isolate validate: weight 1 (grabs 1 slot, ~3-4 GB RSS)
#   full validate:    weight 2 (grabs 2 slots, ~11 GB RSS)
#   6 slots total means: 6 isolates, or 1 full + 4 isolates, or 3 fulls.
#
# Usage in validate.sh / ptt.sh:
#   source "$(dirname "${BASH_SOURCE[0]}")/verus-lock.sh"

VERUS_LOCK_DIR="${VERUS_LOCK_DIR:-/tmp/verus-locks}"
VERUS_LOCK_SLOTS="${VERUS_LOCK_SLOTS:-6}"
VERUS_LOCK_TIMEOUT="${VERUS_LOCK_TIMEOUT:-600}"
VERUS_LOCK_WEIGHT="${VERUS_LOCK_WEIGHT:-1}"

mkdir -p "$VERUS_LOCK_DIR"

_verus_acquired=0

if [ "$VERUS_LOCK_WEIGHT" -le 1 ]; then
    # Weight 1: grab any single slot on fd 9.
    # Try all slots non-blocking first.
    for (( _slot=1; _slot<=VERUS_LOCK_SLOTS; _slot++ )); do
        exec 9>"$VERUS_LOCK_DIR/verus-slot-${_slot}.lock"
        if flock -n 9; then
            _verus_acquired=1
            echo "Acquired verus lock slot ${_slot}/${VERUS_LOCK_SLOTS} (weight 1)"
            break
        fi
    done
    # If all busy, retry with exponential backoff on random slots.
    if [ "$_verus_acquired" -eq 0 ]; then
        _wait=5
        _elapsed=0
        echo "All ${VERUS_LOCK_SLOTS} verus slots busy. Retrying with backoff (max ${VERUS_LOCK_TIMEOUT}s)..."
        while [ "$_elapsed" -lt "$VERUS_LOCK_TIMEOUT" ]; do
            # Pick a random slot to avoid thundering herd on slot 1.
            _slot=$(( (RANDOM % VERUS_LOCK_SLOTS) + 1 ))
            exec 9>"$VERUS_LOCK_DIR/verus-slot-${_slot}.lock"
            if flock -w "$_wait" 9; then
                _verus_acquired=1
                echo "Acquired verus lock slot ${_slot}/${VERUS_LOCK_SLOTS} (weight 1, after ${_elapsed}s)"
                break
            fi
            _elapsed=$((_elapsed + _wait))
            # Exponential backoff: 5, 10, 20, 40, capped at 60.
            _wait=$((_wait * 2))
            [ "$_wait" -gt 60 ] && _wait=60
            echo "  Slot ${_slot} busy, waited ${_elapsed}s so far, next wait ${_wait}s..."
        done
        if [ "$_verus_acquired" -eq 0 ]; then
            echo "ERROR: Could not acquire verus lock after ${VERUS_LOCK_TIMEOUT}s. Aborting."
            exit 1
        fi
    fi
else
    # Weight 2: grab two slots (fd 8 + fd 9). Try non-blocking pairs first.
    for (( _first=1; _first<=VERUS_LOCK_SLOTS-1; _first++ )); do
        _second=$((_first + 1))
        exec 8>"$VERUS_LOCK_DIR/verus-slot-${_first}.lock"
        if flock -n 8; then
            exec 9>"$VERUS_LOCK_DIR/verus-slot-${_second}.lock"
            if flock -n 9; then
                _verus_acquired=2
                echo "Acquired verus lock slots ${_first}+${_second}/${VERUS_LOCK_SLOTS} (weight 2)"
                break
            fi
            # Got first but not second — release first, try next pair.
            exec 8>&-
        fi
    done
    if [ "$_verus_acquired" -lt 2 ]; then
        echo "No slot pair free. Waiting for slots 1+2 (up to ${VERUS_LOCK_TIMEOUT}s)..."
        exec 8>"$VERUS_LOCK_DIR/verus-slot-1.lock"
        if ! flock -w "$VERUS_LOCK_TIMEOUT" 8; then
            echo "ERROR: Could not acquire verus slot 1 after ${VERUS_LOCK_TIMEOUT}s. Aborting."
            exit 1
        fi
        exec 9>"$VERUS_LOCK_DIR/verus-slot-2.lock"
        if ! flock -w "$VERUS_LOCK_TIMEOUT" 9; then
            echo "ERROR: Could not acquire verus slot 2 after ${VERUS_LOCK_TIMEOUT}s. Aborting."
            exit 1
        fi
        echo "Acquired verus lock slots 1+2/${VERUS_LOCK_SLOTS} (weight 2, after waiting)"
    fi
fi

unset _verus_acquired _slot _first _second
