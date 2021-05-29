-- Your SQL goes here
CREATE TABLE IF NOT EXISTS mech_attr
(
    worker_id   INTEGER PRIMARY KEY,
    repair_type TEXT NOT NULL--,
    --CONSTRAINT valid_worker_id FOREIGN KEY (worker_id) REFERENCES worker (worker_id),
    --CONSTRAINT valid_repair_type FOREIGN KEY (repair_type) REFERENCES equipment_type (type)
);