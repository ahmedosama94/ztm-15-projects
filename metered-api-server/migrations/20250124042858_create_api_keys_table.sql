-- Add migration script here
CREATE TABLE api_keys (
    email varchar,
    api_key varchar
);

CREATE UNIQUE INDEX api_keys_email ON api_keys(email);
