-- Your SQL goes here

CREATE TABLE watcher
(
    id       INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    login    VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    salt     BLOB    NOT NULL
);

CREATE TABLE tracker
(
    id        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    status    INTEGER NOT NULL DEFAULT 0,
    latitude  FLOAT   NOT NULL,
    longitude FLOAT   NOT NULL
);

CREATE TABLE monitoring
(
    watcher_id   INTEGER NOT NULL,
    tracker_id   INTEGER NOT NULL,
    tracker_name VARCHAR NOT NULL,
    PRIMARY KEY (watcher_id, tracker_id),
    FOREIGN KEY (watcher_id) REFERENCES watcher (id),
    FOREIGN KEY (tracker_id) REFERENCES tracker (id)
);

CREATE TABLE position
(
    latitude   FLOAT   NOT NULL,
    longitude  FLOAT   NOT NULL,
    tracker_id INTEGER NOT NULL,
    timestamp  BIGINT  NOT NULL,
    PRIMARY KEY (tracker_id, timestamp),
    FOREIGN KEY (tracker_id) REFERENCES tracker (id)
);