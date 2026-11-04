#!/usr/bin/env bash
set -euo pipefail

# Economic Data Importer Script
# Usage: ./import-data.sh [validate|import|summary]

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DATA_DIR="${ROOT_DIR}/economic_data"
DB_PATH="${HOME}/.local/share/macro-journal/macro_journal.db"
SUMMARY_OUTPUT="${DATA_DIR}/CONTROL_SUMMARY.md"

case "${1:-help}" in
    validate)
        echo "🔍 Validating economic data..."
        cargo run --package economic-data-importer -- validate --data-dir "$DATA_DIR"
        ;;
    import)
        echo "📥 Importing economic data..."
        cargo run --package economic-data-importer -- import \
            --data-dir "$DATA_DIR" \
            --db-path "$DB_PATH" \
            --overwrite
        ;;
    summary)
        echo "📊 Generating control summary..."
        cargo run --package economic-data-importer -- generate-summary \
            --data-dir "$DATA_DIR" \
            --output "$SUMMARY_OUTPUT"
        ;;
    full)
        echo "🔄 Running full workflow..."
        echo "1. Validating data..."
        cargo run --package economic-data-importer -- validate --data-dir "$DATA_DIR"
        
        echo "2. Importing data..."
        cargo run --package economic-data-importer -- import \
            --data-dir "$DATA_DIR" \
            --db-path "$DB_PATH" \
            --overwrite
        
        echo "3. Generating summary..."
        cargo run --package economic-data-importer -- generate-summary \
            --data-dir "$DATA_DIR" \
            --output "$SUMMARY_OUTPUT"
        
        echo "✅ Full workflow completed!"
        ;;
    help|*)
        echo "Usage: $0 {validate|import|summary|full}"
        echo ""
        echo "Commands:"
        echo "  validate   - Validate CSV files"
        echo "  import     - Import data into SQLite"
        echo "  summary    - Generate control summary"
        echo "  full       - Run all steps (validate + import + summary)"
        echo ""
        echo "Options:"
        echo "  DATA_DIR   - $DATA_DIR"
        echo "  DB_PATH    - $DB_PATH"
        echo "  OUTPUT    - $SUMMARY_OUTPUT"
        ;;
esac
