-- Your SQL goes here
CREATE TABLE IF NOT EXISTS contract
(
    contract_id    SERIAL PRIMARY KEY,
    cost           BIGINT,
    contract_start TIMESTAMP NOT NULL,
    contract_end   TIMESTAMP NOT NULL
);