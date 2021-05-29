-- Your SQL goes here
CREATE TABLE IF NOT EXISTS lab_attr
(
    worker_id  INTEGER PRIMARY KEY,
    lab_number BIGINT NOT NULL,
    --CONSTRAINT valid_worker_id FOREIGN KEY (worker_id) REFERENCES worker (worker_id),
    CONSTRAINT valid_lab_number CHECK (lab_number > 0)
);