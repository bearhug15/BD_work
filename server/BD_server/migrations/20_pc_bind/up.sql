-- Your SQL goes here
CREATE TABLE IF NOT EXISTS pc_bind
(
    contract_id   INTEGER,
    project_id    INTEGER,
    group_id      INTEGER   NOT NULL,
    head_id       INTEGER   NOT NULL,
    project_start TIMESTAMP NOT NULL,
    project_end   TIMESTAMP NOT NULL,
    eq_list_id    INTEGER   NOT NULL,
    PRIMARY KEY (contract_id, project_id),
    CONSTRAINT valid_contract_id FOREIGN KEY (contract_id) REFERENCES contract (contract_id),
    CONSTRAINT valid_project_id FOREIGN KEY (project_id) REFERENCES project (project_id),
    CONSTRAINT valid_group_id FOREIGN KEY (group_id) REFERENCES groups_bind (group_id),
    CONSTRAINT valid_head_id FOREIGN KEY (head_id) REFERENCES worker (worker_id),
    CONSTRAINT valid_eq_list_id FOREIGN KEY (eq_list_id) REFERENCES eq_group_list (eq_list_id)
);