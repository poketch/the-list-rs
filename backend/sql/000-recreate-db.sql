-- DEV ONLY -- Brute Force recrete DB for live dev and unit testing
BEGIN TRANSACTION;
PRAGMA FOREIGN_KEYS = OFF;
DROP TABLE IF EXISTS list;
DROP TABLE IF EXISTS list_element_status;
PRAGMA FOREIGN_KEYS = ON;
COMMIT;