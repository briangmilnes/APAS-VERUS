#!/bin/bash
# Generate Verus documentation with specifications for APAS-VERUS.
# Adapted from verus source/tools/docs.sh, using the prebuilt release in
# ~/projects/verus/source/target-verus/release (flat layout: verus, verusdoc, z3, lib*.rlib, lib*.so).

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APAS_ROOT="$(dirname "$SCRIPT_DIR")"
VERUS_HOME="${VERUS_HOME:-$HOME/projects/verus/source/target-verus/release}"

# Detect dynamic library extension
if [ "$(uname)" == "Darwin" ]; then
    DYN_LIB_EXT=dylib
elif [ "$(uname)" == "Linux" ]; then
    DYN_LIB_EXT=so
else
    echo "Unsupported OS: $(uname)" >&2
    exit 1
fi

for needed in verusdoc libvstd.rlib libverus_builtin.rlib; do
    if [ ! -e "$VERUS_HOME/$needed" ]; then
        echo "Missing $VERUS_HOME/$needed — unpack a verus release into $VERUS_HOME." >&2
        exit 1
    fi
done

# Create doc output directory
mkdir -p "$APAS_ROOT/target/verusdoc"

echo "Running rustdoc with Verus macros..."
cd "$APAS_ROOT"

RUSTC_BOOTSTRAP=1 VERUSDOC=1 VERUS_Z3_PATH="$VERUS_HOME/z3" rustdoc \
  --crate-name apas_verus \
  --crate-type lib \
  -L "$VERUS_HOME" \
  --extern vstd="$VERUS_HOME/libvstd.rlib" \
  --extern verus_builtin="$VERUS_HOME/libverus_builtin.rlib" \
  --extern verus_builtin_macros="$VERUS_HOME/libverus_builtin_macros.$DYN_LIB_EXT" \
  --extern verus_state_machines_macros="$VERUS_HOME/libverus_state_machines_macros.$DYN_LIB_EXT" \
  --edition=2021 \
  --cfg verus_keep_ghost \
  --cfg verus_keep_ghost_body \
  --cfg 'feature="std"' \
  --cfg 'feature="alloc"' \
  -Zcrate-attr='feature(stmt_expr_attributes)' \
  -Zcrate-attr='feature(negative_impls)' \
  -Zcrate-attr='feature(register_tool)' \
  -Zcrate-attr='feature(rustc_attrs)' \
  -Zcrate-attr='feature(unboxed_closures)' \
  -Zcrate-attr='register_tool(verus)' \
  -Zcrate-attr='register_tool(verifier)' \
  -Zcrate-attr='register_tool(verusfmt)' \
  -Zcrate-attr='allow(internal_features)' \
  -Zcrate-attr='allow(unused_braces)' \
  -Zproc-macro-backtrace \
  -o "$APAS_ROOT/target/verusdoc" \
  src/lib.rs

echo "Running verusdoc post-processor..."
# verusdoc expects docs in a 'doc/' subdirectory
cd "$APAS_ROOT/target"
rm -rf doc
ln -s verusdoc doc
"$VERUS_HOME/verusdoc"

echo ""
echo "Documentation generated at:"
echo "  $APAS_ROOT/target/verusdoc/apas_verus/index.html"
