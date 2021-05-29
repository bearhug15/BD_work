-- Your SQL goes here
CREATE TABLE worker
(
    worker_id       SERIAL PRIMARY KEY,
    firstname       TEXT    NOT NULL,
    secondname      TEXT,
    familyname      TEXT    NOT NULL,
    age             INTEGER NOT NULL,
    salary          BIGINT  NOT NULL,
    department_name TEXT    NOT NULL,
    worker_type     TEXT    NOT NULL,
    CONSTRAINT age_limiter CHECK (age > 0),
    CONSTRAINT salary_limiter CHECK (salary > 0),
    CONSTRAINT valid_department_name FOREIGN KEY (department_name) REFERENCES department (department_name),
    CONSTRAINT valid_worker_type FOREIGN KEY (worker_type) REFERENCES worker_types (worker_type)
);