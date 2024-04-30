-- Your SQL goes here

CREATE TABLE protection (
    protected_id INTEGER NOT NULL,
    protector_id INTEGER NOT NULL,
    protected_name VARCHAR NOT NULL,
    PRIMARY KEY (protected_id, protector_id),
    FOREIGN KEY (protector_id) REFERENCES protector(id),
    FOREIGN KEY (protected_id) REFERENCES protected(id)
)