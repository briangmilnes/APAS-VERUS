#!/bin/bash
# Build only the TOPOS deck: slidesTOPOS.md -> slidesTOPOS.pdf + slidesTOPOS.pptx
set -e
exec "$(dirname "$0")/build.sh" slidesTOPOS
