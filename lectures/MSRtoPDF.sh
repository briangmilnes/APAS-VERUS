#!/bin/bash
set -e
cd "$(dirname "$0")"
pandoc --pdf-engine=lualatex slidesMSR.md -o slidesMSR.pdf
echo "-> $(pwd)/slidesMSR.pdf"
