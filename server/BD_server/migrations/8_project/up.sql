-- Your SQL goes here
CREATE TABLE IF NOT EXISTS project
(
    project_id SERIAL PRIMARY KEY,
    cost       BIGINT ,
    data JSON,
    CONSTRAINT valid_cost CHECK (cost > 0)
);