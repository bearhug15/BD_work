-- Your SQL goes here
CREATE TABLE IF NOT EXISTS working_company
(
    group_id     INTEGER PRIMARY KEY,
    company_name TEXT NOT NULL--,
    --CONSTRAINT valid_group_id FOREIGN KEY (group_id) REFERENCES groups_bind (group_id),
    --CONSTRAINT valid_company FOREIGN KEY (company_name) REFERENCES company (company_name)
);