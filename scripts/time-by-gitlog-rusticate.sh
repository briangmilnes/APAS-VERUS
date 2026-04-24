#!/usr/bin/env bash
# Run time-by-gitlog analysis against ~/projects/rusticate.
set -euo pipefail
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$DIR/time-by-gitlog.sh" "$HOME/projects/rusticate"
