-- Your SQL goes here
CREATE TABLE IF NOT EXISTS constr_attr
(
    worker_id   INTEGER PRIMARY KEY,
    cert_number INTEGER NOT NULL--,
    --CONSTRAINT valid_worker_id FOREIGN KEY (worker_id) REFERENCES worker (worker_id)
);