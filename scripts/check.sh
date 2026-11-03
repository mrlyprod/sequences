#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
cd "$HERE/.."

DATA_DIR="data/scripts/check"
RUFF="uvx ruff@0.17.2"
unit=$(sed -n 's/^unit = "\(.*\)"$/\1/p' bridge.toml 2> /dev/null || true)

step() {
  local name="$1"; shift
  local log="$DATA_DIR/$name.log"
  local start=$SECONDS
  if "$@" > "$log" 2>&1; then
    printf 'ok   %-12s %4ss\n' "$name" "$((SECONDS - start))"
    return 0
  fi
  printf 'FAIL %-12s %4ss\n' "$name" "$((SECONDS - start))"
  tail -n 60 "$log" >&2
  echo "check stopped at $name; full log in $log" >&2
  exit 1
}

lint_py() { $RUFF format --check py && $RUFF check py; }

mkdir -p "$DATA_DIR"
total=$SECONDS
step fmt cargo fmt --all --check
step clippy cargo clippy --workspace --all-targets --all-features -- -D warnings
[ ! -f bridge.toml ] || step bridge scripts/bridge.sh check
[ ! -d py ] || step ruff lint_py
[ ! -d js ] || step wasm scripts/wasm.sh
echo "check passed in $((SECONDS - total))s"
