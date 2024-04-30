-- Your SQL goes here

CREATE TABLE positions_history (
    latitude FLOAT NOT NULL,
    longitude FLOAT NOT NULL,
    protected_id INTEGER NOT NULL,
    timestamp INTEGER NOT NULL,
    PRIMARY KEY (protected_id, timestamp),
    FOREIGN KEY (protected_id) REFERENCES protected(id)
)