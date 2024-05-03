-- Your SQL goes here


INSERT INTO watcher (login, password, salt)
VALUES ('jacques', 'chirac', 'salt_value'),
       ('valery', 'giscard', 'salt_value'),
       ('frederic', 'mitterand', 'salt_value');

INSERT INTO tracker (latitude, longitude)
VALUES (46.7, 3.1),
       (44.9, 6.5),
       (42.6, 5.0);

INSERT INTO monitoring (watcher_id, tracker_id, tracker_name)
VALUES (1, 1, 'Papi'),
       (2, 2, 'Mamie'),
       (3, 3, 'Bébé');

INSERT INTO position (latitude, longitude, tracker_id, timestamp)
VALUES (45.2, 4.3, 1, CURRENT_TIMESTAMP),
       (43.9, 5.7, 2, CURRENT_TIMESTAMP),
       (44.6, 3.8, 3, CURRENT_TIMESTAMP);
