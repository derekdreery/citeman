BEGIN;

-- Generic document
CREATE TABLE document {
    id              INTEGER PRIMARY KEY,
    title           TEXT NOT NULL,
    time_created    TEXT NOT NULL,
    time_updated    TEXT NOT NULL,
    doc_type        TEXT NOT NULL,
    paper_location  TEXT NOT NULL,
    -- optional fields
    year            INTEGER,
    volume          INTEGER,
    "number"        INTEGER,
    pages           TEXT,
};

-- Author
CREATE TABLE author {
    id              INTEGER PRIMARY KEY,
    document_id     INTEGER NOT NULL,
    given_names     TEXT NOT NULL,
    family_name     TEXT NOT NULL,
};

COMMIT;
