-- Generic document
CREATE TABLE documents (
    id              INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title           TEXT NOT NULL,
    time_created    TEXT NOT NULL,
    time_updated    TEXT NOT NULL,
    doc_type        TEXT NOT NULL,
    paper_location  TEXT NOT NULL,
    -- optional fields
    year            INTEGER,
    volume          INTEGER,
    "number"        INTEGER,
    pages           BLOB NOT NULL
);

-- Author
CREATE TABLE authors (
    id              INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    given_names     TEXT NOT NULL,
    family_name     TEXT NOT NULL
);

-- document <-> author many-many table
CREATE TABLE documents_authors (
    document_id     INTEGER NOT NULL,
    author_id       INTEGER NOT NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id),
    FOREIGN KEY (author_id) REFERENCES authors(id),
    PRIMARY KEY (document_id, author_id)
) WITHOUT ROWID;
