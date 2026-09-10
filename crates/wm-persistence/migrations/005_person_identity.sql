CREATE TABLE identity_traits (
    worker_id TEXT PRIMARY KEY REFERENCES workers(id),
    revision INTEGER NOT NULL CHECK(revision >= 0),
    ledger TEXT NOT NULL CHECK(json_valid(ledger))
) STRICT;
