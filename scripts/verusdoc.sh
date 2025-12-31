#!/bin/bash
# Generate Verus documentation with specifications for APAS-VERUS
# Adapted from ~/projects/verus/source/tools/docs.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APAS_ROOT="$(dirname "$SCRIPT_DIR")"
VERUS_SOURCE="$HOME/projects/verus/source"

# Detect dynamic library extension
if [ "$(uname)" == "Darwin" ]; then
    DYN_LIB_EXT=dylib
elif [ "$(uname)" == "Linux" ]; then
    DYN_LIB_EXT=so
else
    echo "Unsupported OS: $(uname)" >&2
    exit 1
fi

# Build verusdoc and vstd debug if not present
cd "$VERUS_SOURCE"
. "$HOME/projects/verus/tools/activate"

if [ ! -f "$VERUS_SOURCE/target/debug/verusdoc" ]; then
    echo "Building verusdoc..."
    vargo build -p verusdoc
fi

if [ ! -f "$VERUS_SOURCE/target-verus/debug/libvstd.rlib" ]; then
    echo "Building vstd (debug, no verify)..."
    vargo build --vstd-no-verify
fi

cd "$APAS_ROOT"

# Create doc output directory
mkdir -p "$APAS_ROOT/target/verusdoc"

echo "Running rustdoc with Verus macros..."
cd "$APAS_ROOT"

# Use debug builds like verus docs.sh does
VERUS_TARGET="$VERUS_SOURCE/target-verus/debug"

RUSTC_BOOTSTRAP=1 VERUSDOC=1 VERUS_Z3_PATH="$VERUS_SOURCE/z3" rustdoc \
  --crate-name apas_verus \
  --crate-type lib \
  -L "$VERUS_TARGET" \
  --extern vstd="$VERUS_TARGET/libvstd.rlib" \
  --extern verus_builtin="$VERUS_TARGET/libverus_builtin.rlib" \
  --extern verus_builtin_macros="$VERUS_TARGET/libverus_builtin_macros.$DYN_LIB_EXT" \
  --extern verus_state_machines_macros="$VERUS_TARGET/libverus_state_machines_macros.$DYN_LIB_EXT" \
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
"$VERUS_SOURCE/target/debug/verusdoc"

echo ""
echo "Documentation generated at:"
echo "  $APAS_ROOT/target/verusdoc/apas_verus/index.html"

