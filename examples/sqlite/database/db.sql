PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE item_states ( id INTEGER PRIMARY KEY, label TEXT NOT NULL UNIQUE );
INSERT INTO item_states VALUES(1,'todo');
INSERT INTO item_states VALUES(2,'in_progress');
INSERT INTO item_states VALUES(3,'completed');
INSERT INTO item_states VALUES(4,'parked');
INSERT INTO item_states VALUES(5,'archived');
COMMIT;
