-- Generic document
CREATE TABLE documents (
    id              INTEGER     PRIMARY KEY,
    title           TEXT        NOT NULL,
    time_created    TEXT        NOT NULL,
    time_updated    TEXT        NOT NULL,
    doc_type        TEXT        NOT NULL,
    paper_location  TEXT        NOT NULL
);

-- Journal articles
CREATE TABLE articles (
    document_id     INTEGER     PRIMARY KEY NOT NULL,
    journal_id      INTEGER     NOT NULL,
    year            INTEGER,
    month           INTEGER,
    volume          INTEGER,
    "number"        INTEGER,
    pages           BLOB        NOT NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id)
    FOREIGN KEY (journal_id) REFERENCES journals(id)
) WITHOUT ROWID;

-- Author
CREATE TABLE authors (
    id              INTEGER     PRIMARY KEY AUTOINCREMENT NOT NULL,
    given_names     TEXT        NOT NULL,
    family_name     TEXT        NOT NULL
);

-- document <-> author many-many table
CREATE TABLE documents_authors (
    document_id     INTEGER     NOT NULL,
    author_id       INTEGER     NOT NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id),
    FOREIGN KEY (author_id) REFERENCES authors(id),
    PRIMARY KEY (document_id, author_id)
) WITHOUT ROWID;

-- journals
CREATE TABLE journals (
    id          INTEGER         PRIMARY KEY,
    name        TEXT            NOT NULL,
    short_name  TEXT,
);
