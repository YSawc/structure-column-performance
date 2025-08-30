-- Column type storage (normalized table structure)
CREATE TABLE users_column (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    age INTEGER NOT NULL,
    bio TEXT,
    avatar_url TEXT,
    preferences TEXT, -- JSON as TEXT in SQLite
    social_links TEXT, -- JSON as TEXT in SQLite
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- JSON type storage (denormalized table structure)
CREATE TABLE users_json (
    id TEXT PRIMARY KEY,
    data TEXT NOT NULL, -- JSON as TEXT in SQLite
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Create indexes
CREATE INDEX idx_users_column_created_at ON users_column(created_at);
CREATE INDEX idx_users_column_email ON users_column(email);
CREATE INDEX idx_users_column_age ON users_column(age);

CREATE INDEX idx_users_json_created_at ON users_json(created_at);
CREATE INDEX idx_users_json_email ON users_json(json_extract(data, '$.email'));
CREATE INDEX idx_users_json_age ON users_json(json_extract(data, '$.age'));