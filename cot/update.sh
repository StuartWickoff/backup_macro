#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PYTHON="$ROOT_DIR/.venv/bin/python"

if [[ ! -x "$PYTHON" ]]; then
  echo "Erreur : environnement Python COT introuvable : $PYTHON" >&2
  echo "Créez-le avec : python3 -m venv .venv && .venv/bin/python -m pip install -r cot/requirements.txt" >&2
  exit 1
fi

"$PYTHON" "$ROOT_DIR/cot/cot_dashboard.py" \
  --smooth \
  --web-assets-dir "$ROOT_DIR/apps/web/public/cot"
