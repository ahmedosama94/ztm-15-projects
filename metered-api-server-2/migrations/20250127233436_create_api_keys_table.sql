-- Add migration script here
CREATE TABLE api_keys (
    id integer primary key,
    email varchar(255),
    api_key varchar(512)
);

CREATE UNIQUE INDEX api_keys_email ON api_keys(email);
