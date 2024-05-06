-- Your SQL goes here


INSERT INTO watcher (login, password, salt)
-- admin adminPassword
-- jacques chirac
-- valery giscard
-- frederic mitterand
VALUES ('admin', '$argon2id$v=19$m=19456,t=2,p=1$c2FsdF92YWx1ZQ$7TBumjJiIKYJnl53BFdFBRh15XL4SSnr5NxMscJDsE8', 'salt_value'),
       ('jacques', '$argon2id$v=19$m=19456,t=2,p=1$c2FsdF92YWx1ZQ$FnKEb0tRczVm1fNkwRRgOMQi9XWsFPwHBKL8ctnFrlQ', 'salt_value'),
       ('valery', '$argon2id$v=19$m=19456,t=2,p=1$c2FsdF92YWx1ZQ$zCAyibf1UBU8ivwhJthaR3QuVZEkSipmfK7dwhI/a3k', 'salt_value'),
       ('frederic', '$argon2id$v=19$m=19456,t=2,p=1$c2FsdF92YWx1ZQ$bIyWigTg4tClXp7AW6jNioanQjrUVdxHdtxkQYUfr44', 'salt_value');

INSERT INTO tracker (latitude, longitude)
VALUES (46.7, 3.1),
       (44.9, 6.5),
       (42.6, 5.0);

INSERT INTO monitoring (watcher_id, tracker_id, tracker_name)
VALUES (4, 1, 'Papi'),
       (2, 2, 'Mamie'),
       (3, 3, 'Bébé');

INSERT INTO position (latitude, longitude, tracker_id, timestamp)
VALUES (45.2, 4.3, 1, strftime('%s', 'now')),
       (43.9, 5.7, 2, strftime('%s', 'now')),
       (43.8, 5.6, 2, strftime('%s', 'now') + 2),
       (44.6, 3.8, 3, strftime('%s', 'now'));
