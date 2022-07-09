-- https://sqlite.org/datatype3.html#storage_classes_and_datatypes
-- https://www.sqlite.org/autoinc.html
CREATE TABLE adapter (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL UNIQUE
);
INSERT INTO adapter (uuid, name)
VALUES("55e0af45-48df-420e-a0eb-134ea1e806db", "json"),
    ("6a4a11f9-682c-43c2-9385-fde30a14350b", "rust");
CREATE TABLE report (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    project TEXT,
    testbed TEXT,
    adapter_id INTEGER NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    FOREIGN KEY (adapter_id) REFERENCES adapter (id)
);