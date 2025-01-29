CREATE TABLE api_requests (
    id INTEGER PRIMARY KEY,
    api_key_id INTEGER NOT NULL,
    lines_of_code INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY(api_key_id) REFERENCES api_keys(id)
);

CREATE INDEX api_requests_api_key_id_created_at ON api_requests(api_key_id, created_at);
