ALTER TABLE entries
    ADD COLUMN currency_code TEXT NOT NULL DEFAULT 'UNASSIGNED';

CREATE INDEX IF NOT EXISTS idx_entries_currency_updated
    ON entries (currency_code, updated_at DESC, id DESC);
