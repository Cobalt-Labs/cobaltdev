-- File metadata + basic user support
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS files (
    id TEXT PRIMARY KEY,                    -- uuid
    filename TEXT NOT NULL,
    storage_path TEXT NOT NULL,             -- relative path on HDD
    owner_username TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    checksum TEXT NOT NULL,                 -- blake3
    uploaded_at TEXT NOT NULL,
    FOREIGN KEY (owner_username) REFERENCES users(username)
);

-- Insert default user (you)
INSERT OR IGNORE INTO users (username, created_at) 
VALUES ('ibrahim3595', datetime('now'));