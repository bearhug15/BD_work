-- Your SQL goes here
CREATE TABLE IF NOT EXISTS eq_group
(
    id         SERIAL PRIMARY KEY,
    eq_list_id INTEGER NOT NULL,
    eq_id      INTEGER NOT NULL,
    CONSTRAINT valid_eq_list_id FOREIGN KEY (eq_list_id) REFERENCES eq_group_list (eq_list_id),
    CONSTRAINT valid_eq_id FOREIGN KEY (eq_id) REFERENCES equipment (eq_id)
);