
-- ListElementStatus Enum
CREATE TABLE IF NOT EXISTS list_element_status (
    status TEXT NOT NULL PRIMARY KEY,
    id INTEGER
);

INSERT INTO list_element_status (status, id) VALUES ('Open', 1);
INSERT INTO list_element_status (status, id) VALUES ('Closed', 2);
INSERT INTO list_element_status (status, id) VALUES ('Archived', 3);

-- List 
CREATE TABLE IF NOT EXISTS list (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    notes TEXT,
    tags TEXT,
    ctime TEXT NOT NULL DEFAULT (datetime('now')),
    mtime TEXT,    --modified time 
    status TEXT NOT NULL DEFAULT 'Open',
    FOREIGN KEY(status) REFERENCES list_element_status(status)
);
