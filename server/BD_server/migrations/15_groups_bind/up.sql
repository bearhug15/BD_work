-- Your SQL goes here
CREATE TABLE IF NOT EXISTS groups_bind
(
    group_id   SERIAL PRIMARY KEY,
    cost       BIGINT NOT NULL,
    group_type TEXT   NOT NULL--,
    --CONSTRAINT valid_group_type FOREIGN KEY (group_type) REFERENCES group_types (group_type)
);