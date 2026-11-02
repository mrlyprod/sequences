#!/usr/bin/env bash

set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
cd "$HERE/.."

DATA_DIR="data/scripts/check"
LEAKS='/Users/|Developer/|AKIA[0-9A-Z]{16}|npm_[A-Za-z0-9]{36}|pypi-[A-Za-z0-9_-]{40,}|cio[A-Za-z0-9]{32}'
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

leak() {
  local hits code=0
  [ -n "${MRLY_LEAK:-}" ] && [ -f "$MRLY_LEAK" ] || { echo "leak: no list in MRLY_LEAK, the push tool runs it"; return 0; }
  hits="$( { echo "$LEAKS"; sed '/^$/d' "$MRLY_LEAK"; } | git grep --untracked -nIE -f - -- ':!scripts/check.sh')" || code=$?
  [ "$code" -le 1 ] || { echo "git grep failed with $code" >&2; return 1; }
  [ -z "$hits" ] || { echo "$hits"; echo "private paths or token shapes in the tree" >&2; return 1; }
}

lint_py() { $RUFF format --check py && $RUFF check py; }
leaf_cli() { cargo test -p "mrly-$unit-cli"; }
leaf_js() { (cd js && bun test); }
leaf_node() { node js/node.smoke.js; }
leaf_py() { uv run --no-project --with pytest --with ./py pytest py; }

mkdir -p "$DATA_DIR"
total=$SECONDS
step leak leak
step fmt cargo fmt --all --check
step clippy cargo clippy --workspace --all-targets --all-features -- -D warnings
step test cargo test
step doc env RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --workspace --all-features
[ ! -f bridge.toml ] || step bridge scripts/bridge.sh check
[ ! -d py ] || step ruff lint_py
[ ! -d js ] || step wasm scripts/wasm.sh
[ ! -d cli ] || step leaf-cli leaf_cli
[ ! -d js ] || step leaf-js leaf_js
[ ! -d js ] || step leaf-node leaf_node
[ ! -d py ] || step leaf-py leaf_py
echo "check passed in $((SECONDS - total))s"
