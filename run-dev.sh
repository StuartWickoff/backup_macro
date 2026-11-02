#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

if ! command -v cargo >/dev/null 2>&1; then
  echo "Erreur : cargo est introuvable. Installe Rust avec rustup."
  exit 1
fi
if ! command -v trunk >/dev/null 2>&1; then
  echo "Erreur : trunk est introuvable. Installe-le avec: cargo install --locked trunk"
  exit 1
fi

cargo run -p macro-journal-server &
SERVER_PID=$!
cleanup() {
  kill "$SERVER_PID" 2>/dev/null || true
  wait "$SERVER_PID" 2>/dev/null || true
}
trap cleanup EXIT INT TERM

cd "$ROOT_DIR/apps/web"
trunk serve --address 127.0.0.1 --port 8080
