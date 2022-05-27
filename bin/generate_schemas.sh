#/bin/bash
set -euo pipefail
set -x

(
cd "$(dirname "$0")/.."

git ls-files '*_schema.rs' | while read -r f; do
    ( cd "$(dirname "$f")/.." && cargo run --locked --example "$(basename -s .rs "$f")")
done
)
