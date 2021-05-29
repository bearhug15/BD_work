-- Your SQL goes here
CREATE TABLE IF NOT EXISTS groups
(
    group_id  INTEGER NOT NULL,
    worker_id INTEGER NOT NULL--,
    --CONSTRAINT valid_group_id FOREIGN KEY (group_id) REFERENCES groups_bind (group_id),
    --CONSTRAINT valid_worker_id FOREIGN KEY (worker_id) REFERENCES worker (worker_id)
);