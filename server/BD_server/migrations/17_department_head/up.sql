-- Your SQL goes here
CREATE TABLE IF NOT EXISTS department_head
(
    department_name TEXT PRIMARY KEY,
    worker_id       INTEGER NOT NULL,
    CONSTRAINT valid_department_name FOREIGN KEY (department_name) REFERENCES department (department_name),
    CONSTRAINT valid_worker_id FOREIGN KEY (worker_id) REFERENCES worker (worker_id)
);