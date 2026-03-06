#!/bin/bash
# blockers.sh — Per-file chapter blocking report with dependency graph and proof status.
#
# Produces:
#   1. plans/chapter-blockers.md     — per-file table
#   2. src/ChapNN/plans/blockers.md  — per-chapter detail
#
# Usage: scripts/blockers.sh
#
# Columns: #, Chap, File, Verusified, Specs, Verified, Holes, Blocked By, Blocks

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$ROOT/src"
LIBRS="$SRC/lib.rs"
PLANS="$ROOT/plans"
REPORT="$PLANS/chapter-blockers.md"

mkdir -p "$PLANS"

# 1. Run holes.sh once, cache output.
HOLES_CACHE=$(mktemp)
trap 'rm -f "$HOLES_CACHE"' EXIT
"$ROOT/scripts/holes.sh" 2>&1 > "$HOLES_CACHE"

# 2. Extract all files from lib.rs: chapter, module name, active/commented.
#    Output: ChapNN|ModuleName|active  or  ChapNN|ModuleName|commented
extract_files() {
    local in_chap=""
    local brace_depth=0
    while IFS= read -r line; do
        # Detect chapter block start: "pub mod ChapNN {"
        if [[ "$line" =~ ^[[:space:]]*(\#\[cfg[^\]]*\])?[[:space:]]*pub\ mod\ (Chap[0-9]+)\ \{ ]]; then
            in_chap="${BASH_REMATCH[2]}"
            brace_depth=1
            continue
        fi
        # Track braces inside chapter block.
        if [ -n "$in_chap" ]; then
            if [[ "$line" == "}" ]]; then
                brace_depth=$((brace_depth - 1))
                if [ "$brace_depth" -le 0 ]; then
                    in_chap=""
                    continue
                fi
            fi
            # Active pub mod line.
            if [[ "$line" =~ ^[[:space:]]*pub\ mod\ ([A-Za-z0-9_]+)\; ]]; then
                echo "${in_chap}|${BASH_REMATCH[1]}|active"
            # Cfg-gated pub mod line (next line has pub mod).
            elif [[ "$line" =~ ^[[:space:]]*\#\[cfg ]]; then
                : # skip, the pub mod on next line will be caught
            # Commented-out pub mod line.
            elif [[ "$line" =~ ^[[:space:]]*//[[:space:]]*pub\ mod\ ([A-Za-z0-9_]+) ]]; then
                echo "${in_chap}|${BASH_REMATCH[1]}|commented"
            fi
        fi
    done < "$LIBRS"
}

FILES_LIST=$(mktemp)
extract_files > "$FILES_LIST"

# 3. Build file-level import graph.
#    For each active file, find what ChapNN/Module it imports.
declare -A FILE_IMPORTS  # key=Chap/Module, value=space-separated list of Chap/Module it imports
declare -A FILE_BLOCKS   # key=Chap/Module, value=space-separated list of Chap/Module that import it

while IFS='|' read -r chap mod status; do
    [ "$status" != "active" ] && continue
    filepath="$SRC/${chap}/${mod}.rs"
    [ ! -f "$filepath" ] && continue

    key="${chap}/${mod}"
    # Find "use crate::ChapNN::Module::*" imports (cross-chapter only).
    imports=$(grep -oP 'use crate::(Chap\d+)::(\w+)' "$filepath" 2>/dev/null \
        | sed 's/use crate:://' | sed 's/::/\//' | sort -u \
        | grep -v "^${chap}/${mod}$" || true)

    FILE_IMPORTS["$key"]="$imports"

    for imp in $imports; do
        FILE_BLOCKS["$imp"]="${FILE_BLOCKS[$imp]:-} $key"
    done
done < "$FILES_LIST"

# 4. Count holes per file.
count_file_holes() {
    local chap="$1" mod="$2"
    grep 'info:' "$HOLES_CACHE" | grep -c "${chap}/${mod}.rs" 2>/dev/null || true
}

# 5. Classify spec strength per file from fn-impls JSON.
#    high = >80% fns have specs, med = 40-80%, low = <40%, n/a = no data
classify_specs() {
    local chap="$1" mod="$2"
    local json="$SRC/${chap}/analyses/veracity-review-module-fn-impls.json"
    if [ ! -f "$json" ]; then
        echo "?"
        return
    fi
    # Count fns with specs (Unk) vs total for this module.
    local with_spec no_spec total
    # Use python for reliable JSON parsing.
    read -r with_spec no_spec <<< $(python3 -c "
import json, sys
with open('$json') as f:
    data = json.load(f)
ws = sum(1 for d in data if d.get('file','').endswith('${mod}.rs') and d.get('spec_strength') != 'none')
ns = sum(1 for d in data if d.get('file','').endswith('${mod}.rs') and d.get('spec_strength') == 'none')
print(ws, ns)
" 2>/dev/null || echo "0 0")
    total=$((with_spec + no_spec))
    if [ "$total" -eq 0 ]; then
        echo "?"
        return
    fi
    local pct=$((with_spec * 100 / total))
    if [ "$pct" -ge 80 ]; then
        echo "high"
    elif [ "$pct" -ge 40 ]; then
        echo "med"
    else
        echo "low"
    fi
}

# 6. Format file references: "NN/File" style (compact).
fmt_deps() {
    local raw="$1"
    if [ -z "$raw" ]; then
        echo "—"
        return
    fi
    # Shorten "ChapNN/Module" to "NN/Module", strip empty entries.
    echo "$raw" | tr ' ' '\n' | sed '/^$/d' | sort -u | sed 's/Chap//' | tr '\n' ',' | sed 's/^,//;s/,$//' | sed 's/,/, /g'
}

# 7. Per-chapter detail files (original behavior).
cd "$SRC"
declare -A CHAP_BLOCKS
for dir in Chap*/; do
    chap="${dir%/}"
    downstream=$(grep -rl "use crate::${chap}::" Chap*/*.rs 2>/dev/null \
        | sed 's|/.*||' | sort -u | grep -v "^${chap}$" || true)
    CHAP_BLOCKS["$chap"]="$downstream"
done

for dir in Chap*/; do
    chap="${dir%/}"
    mkdir -p "${chap}/plans"
    file="${chap}/plans/blockers.md"
    downstream="${CHAP_BLOCKS[$chap]:-}"
    if [ -z "$downstream" ]; then
        cat > "$file" << EOF
# ${chap} Blockers

No downstream chapters import from ${chap}.
EOF
    else
        {
            echo "# ${chap} Blockers"
            echo ""
            echo "Proving ${chap} blocks these downstream chapters:"
            echo ""
            for dep in $downstream; do
                echo "- ${dep}"
            done
        } > "$file"
    fi
done
cd "$ROOT"

# 8. Spec strength: use fn-impls JSON "spec_strength" field.
#    The JSON has "unknown" for fns with specs (requires/ensures present)
#    and we count how many have it vs total.
#    Actually the JSON spec_strength is always "unknown" — use requires/ensures count instead.
classify_specs_grep() {
    local chap="$1" mod="$2"
    local filepath="$SRC/${chap}/${mod}.rs"
    [ ! -f "$filepath" ] && echo "—" && return

    local spec_lines total_fns
    spec_lines=$(grep -cE '^\s*(requires|ensures)' "$filepath" 2>/dev/null || true)
    total_fns=$(grep -cE '^\s*(pub )?(fn |proof fn |spec fn |open spec fn )' "$filepath" 2>/dev/null || true)

    if [ "$total_fns" -eq 0 ]; then
        echo "—"
    elif [ "$spec_lines" -eq 0 ]; then
        echo "low"
    else
        local ratio=$((spec_lines * 100 / total_fns))
        if [ "$ratio" -ge 150 ]; then
            echo "high"
        elif [ "$ratio" -ge 50 ]; then
            echo "med"
        else
            echo "low"
        fi
    fi
}

# 9. Generate the report.
{
    cat << 'STYLE'
<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>
STYLE
    echo ""
    echo "# Chapter Blocking Report"
    echo ""
    echo "Generated: $(date +%Y-%m-%d)"
    echo ""
    echo "| # | Chap | File | Verusified | Specs | Verified | Holes | Blocked By | Blocks |"
    echo "|---|------|------|------------|-------|----------|-------|------------|--------|"

    row=1
    while IFS='|' read -r chap mod status; do
        num="${chap#Chap}"
        key="${chap}/${mod}"

        if [ "$status" = "active" ]; then
            verusified="yes"
            verified="yes"
            holes=$(count_file_holes "$chap" "$mod")
            [ -z "$holes" ] && holes=0
            specs=$(classify_specs_grep "$chap" "$mod")
            # Get raw lists (one entry per line).
            raw_blocked_by=$(echo "${FILE_IMPORTS[$key]:-}" | tr ' ' '\n' | sed '/^$/d' | sort -u | sed 's/Chap//')
            raw_blocks=$(echo "${FILE_BLOCKS[$key]:-}" | tr ' ' '\n' | sed '/^$/d' | sort -u | sed 's/Chap//')
        else
            verusified="no"
            verified="no"
            holes="—"
            specs="—"
            raw_blocked_by=""
            raw_blocks=""
        fi

        # Build parallel arrays from blocked_by and blocks.
        mapfile -t bb_arr <<< "$raw_blocked_by"
        mapfile -t bl_arr <<< "$raw_blocks"
        # Treat single empty entry as empty array.
        [ "${#bb_arr[@]}" -eq 1 ] && [ -z "${bb_arr[0]}" ] && bb_arr=()
        [ "${#bl_arr[@]}" -eq 1 ] && [ -z "${bl_arr[0]}" ] && bl_arr=()

        local_max=${#bb_arr[@]}
        [ "${#bl_arr[@]}" -gt "$local_max" ] && local_max=${#bl_arr[@]}
        [ "$local_max" -eq 0 ] && local_max=1

        for ((i=0; i<local_max; i++)); do
            bb="${bb_arr[$i]:-—}"
            bl="${bl_arr[$i]:-—}"
            [ -z "$bb" ] && bb="—"
            [ -z "$bl" ] && bl="—"
            if [ "$i" -eq 0 ]; then
                printf "| %d | %s | %s | %s | %s | %s | %s | %s | %s |\n" \
                    "$row" "$num" "$mod" "$verusified" "$specs" "$verified" "$holes" "$bb" "$bl"
            else
                printf "| | | | | | | | %s | %s |\n" "$bb" "$bl"
            fi
        done
        row=$((row + 1))
    done < "$FILES_LIST"

    echo ""
    echo "**Verusified**: code inside verus! blocks with specs"
    echo ""
    echo "**Specs**: high (>1.5 spec lines/fn), med (0.5-1.5), low (<0.5)"
    echo ""
    echo "**Blocked By / Blocks**: NN/Module format (chapter/file)"
} > "$REPORT"

rm -f "$FILES_LIST"

total=$(wc -l < "$REPORT")
echo "Chapter blocking report: $REPORT ($((total)) lines)"
echo "Per-chapter blockers.md: $(ls -d src/Chap*/plans/blockers.md | wc -l) chapters."
