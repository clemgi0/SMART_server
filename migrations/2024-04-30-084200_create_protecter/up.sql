-- Your SQL goes here

CREATE TABLE protector (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    login VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    salt BLOB NOT NULL
)