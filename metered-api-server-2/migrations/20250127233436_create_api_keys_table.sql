CREATE TABLE api_keys (
    id INTEGER PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    api_key VARCHAR(512) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX api_keys_email ON api_keys(email);
CREATE UNIQUE INDEX api_keys_api_key ON api_keys(api_key);
