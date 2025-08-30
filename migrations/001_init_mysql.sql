-- Column type storage (normalized table structure)
CREATE TABLE users_column (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    age INTEGER NOT NULL,
    bio TEXT,
    avatar_url VARCHAR(500),
    preferences JSON,
    social_links JSON,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- JSON type storage (denormalized table structure)
CREATE TABLE users_json (
    id CHAR(36) PRIMARY KEY,
    data JSON NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX idx_users_column_created_at ON users_column(created_at);
CREATE INDEX idx_users_column_email ON users_column(email);
CREATE INDEX idx_users_column_age ON users_column(age);

CREATE INDEX idx_users_json_created_at ON users_json(created_at);
CREATE INDEX idx_users_json_email ON users_json((CAST(JSON_EXTRACT(data, '$.email') AS CHAR(255))));
CREATE INDEX idx_users_json_age ON users_json((CAST(JSON_EXTRACT(data, '$.age') AS UNSIGNED)));