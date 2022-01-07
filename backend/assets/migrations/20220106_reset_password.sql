CREATE TABLE IF NOT EXISTS reset (
    reset_id TEXT PRIMARY KEY,
    reset_code TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    expire_at INTEGER NOT NULL
);