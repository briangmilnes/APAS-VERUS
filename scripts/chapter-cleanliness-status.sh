#!/usr/bin/env bash
# chapter-cleanliness-status.sh — Show clean/holed chapters and dependency blockers.
# Reads from analyses/veracity-review-verus-proof-holes.log (run all-holes-by-chap.sh first).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
LOG="$ROOT/analyses/veracity-review-verus-proof-holes.log"
OUT="$ROOT/analyses/chapter-cleanliness-status.log"

if [[ ! -f "$LOG" ]]; then
    echo "ERROR: $LOG not found. Run scripts/all-holes-by-chap.sh first." >&2
    exit 1
fi

gawk '
# Section 4.1 data lines: "   1  Chap43  (145 holes, 11 files)"
/^\s+[0-9]+\s+(Chap[0-9]+|Concurrency|ParaPairs)\s+\([0-9]+ holes?/ {
    if (match($0, /(Chap[0-9]+|Concurrency|ParaPairs)\s+\(([0-9]+) holes?, ([0-9]+) files?\)/, m)) {
        chap = m[1]
        chapters[chap] = 1
        holes[chap] = m[2] + 0
        files[chap] = m[3] + 0
    }
}

# Section 2.2 data lines: "src/ChapNN/File.rs  depends upon ..."
/^src\/(Chap[0-9]+|Concurrency|ParaPairs).*depends upon/ {
    if (match($0, /^src\/([A-Za-z]+[0-9]*)\//, m)) {
        chap = m[1]
        if (!(chap in chapters)) {
            chapters[chap] = 1
            holes[chap] = 0
        }
        dep_files[chap]++
    }
    # Also handle non-slash modules like Concurrency.rs, ParaPairs.rs
}
/^src\/(Concurrency|ParaPairs)\.rs.*depends upon/ {
    if (match($0, /^src\/([A-Za-z]+)\.rs/, m)) {
        chap = m[1]
        if (!(chap in chapters)) {
            chapters[chap] = 1
            holes[chap] = 0
        }
        dep_files[chap]++
    }
}

/depends upon holed modules:/ {
    # Get chapter from beginning of line
    if (match($0, /^src\/([A-Za-z]+[0-9]*)[\/.:]/, m)) {
        chap = m[1]
    } else {
        next
    }
    line = $0
    sub(/.*depends upon holed modules: */, "", line)
    gsub(/,/, " ", line)
    n = split(line, deps_arr, " ")
    for (i = 1; i <= n; i++) {
        dep = deps_arr[i]
        if (dep == "") continue
        if (match(dep, /^([A-Za-z]+[0-9]*)::/, dm)) {
            dep_chap = dm[1]
            if (dep_chap != chap) {
                key = chap SUBSEP dep
                if (!(key in seen_dep)) {
                    seen_dep[key] = 1
                    if (chap in ext_deps)
                        ext_deps[chap] = ext_deps[chap] ", " dep
                    else
                        ext_deps[chap] = dep
                }
            } else {
                key = chap SUBSEP dep
                if (!(key in seen_int)) {
                    seen_int[key] = 1
                    if (chap in int_deps)
                        int_deps[chap] = int_deps[chap] ", " dep
                    else
                        int_deps[chap] = dep
                }
            }
        }
    }
}

# Section 3 global total
/^Holes Found: [0-9]+ total/ {
    if (match($0, /([0-9]+) total/, m)) {
        global_holes = m[1] + 0
    }
}

END {
    # Collect and sort chapters
    n_chaps = 0
    for (c in chapters) {
        n_chaps++
        chap_list[n_chaps] = c
    }
    for (i = 2; i <= n_chaps; i++) {
        kv = chap_list[i]
        j = i - 1
        while (j > 0 && chap_list[j] > kv) {
            chap_list[j+1] = chap_list[j]
            j--
        }
        chap_list[j+1] = kv
    }

    # Use dep_files count where Section 4.1 had 0 files (clean chapters)
    for (i = 1; i <= n_chaps; i++) {
        c = chap_list[i]
        if (!(c in files) && (c in dep_files)) files[c] = dep_files[c]
        if (!(c in files)) files[c] = 0
    }

    # Totals
    total_h = 0; n_clean = 0; n_holed = 0; total_f = 0
    for (i = 1; i <= n_chaps; i++) {
        c = chap_list[i]
        total_h += holes[c]
        total_f += files[c]
        if (holes[c] == 0) n_clean++; else n_holed++
    }

    auth_holes = (global_holes > 0) ? global_holes : total_h
    printf "Chapter Status — %d chapters, %d clean, %d holed, %d holes (global), %d modules\n\n", n_chaps, n_clean, n_holed, auth_holes, total_f

    # Clean chapters
    printf "CLEAN CHAPTERS (%d)\n", n_clean
    printf "  %-14s %5s\n", "Chapter", "Files"
    printf "  %-14s %5s\n", "--------------", "-----"
    for (i = 1; i <= n_chaps; i++) {
        c = chap_list[i]
        if (holes[c] == 0) printf "  %-14s %5d\n", c, files[c]
    }

    # Holed chapters
    printf "\nHOLED CHAPTERS (%d) — %d holes\n", n_holed, total_h
    printf "  %-14s %5s %5s  %-8s  %s\n", "Chapter", "Holes", "Files", "ClnDeps?", "Blocked by (external holed modules)"
    printf "  %-14s %5s %5s  %-8s  %s\n", "--------------", "-----", "-----", "--------", "-----------------------------------"
    for (i = 1; i <= n_chaps; i++) {
        c = chap_list[i]
        if (holes[c] > 0) {
            if (c in ext_deps) {
                status = "NO"
                blocked = ext_deps[c]
            } else if (c in int_deps) {
                status = "internal"
                blocked = int_deps[c]
            } else {
                status = "YES"
                blocked = ""
            }
            printf "  %-14s %5d %5d  %-8s  %s\n", c, holes[c], files[c], status, blocked
        }
    }

    # Dependency chain — chapter level
    printf "\nDEPENDENCY CHAIN (chapter-level, external only)\n"
    printf "  %-14s  %s\n", "Chapter", "Blocked by chapters"
    printf "  %-14s  %s\n", "--------------", "-------------------"
    for (i = 1; i <= n_chaps; i++) {
        c = chap_list[i]
        if (c in ext_deps) {
            delete dep_chaps
            n_d = split(ext_deps[c], darr, ", ")
            dep_str = ""
            for (d = 1; d <= n_d; d++) {
                if (match(darr[d], /^([A-Za-z]+[0-9]*)::/, dm2)) {
                    dc = dm2[1]
                    if (!(dc in dep_chaps)) {
                        dep_chaps[dc] = 1
                        if (dep_str == "") dep_str = dc
                        else dep_str = dep_str ", " dc
                    }
                }
            }
            printf "  %-14s  %s\n", c, dep_str
        }
    }
}
' "$LOG" | tee "$OUT"

echo ""
echo "Full output: $OUT"
