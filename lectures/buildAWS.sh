#!/bin/bash
# Build only the AWS deck: slidesAWS.md -> slidesAWS.pdf + slidesAWS.pptx
set -e
exec "$(dirname "$0")/build.sh" slidesAWS
