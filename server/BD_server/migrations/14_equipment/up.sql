-- Your SQL goes here
CREATE TABLE IF NOT EXISTS equipment
(
    eq_id           SERIAL PRIMARY KEY,
    name            TEXT NOT NULL,
    department_name TEXT,
    type            TEXT NOT NULL,
    CONSTRAINT valid_department_name FOREIGN KEY (department_name) REFERENCES department (department_name),
    CONSTRAINT valid_type FOREIGN KEY (type) REFERENCES equipment_type (type)
);