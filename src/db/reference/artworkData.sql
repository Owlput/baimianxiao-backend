CREATE TABLE artworkData(
    "id" VARCHAR(16) NOT NULL PRIMARY KEY,
    "title" VARCHAR(50) NOT NULL,
    "author_name" VARCHAR(50) NOT NULL,
    "author_id" VARCHAR(16) REFERENCES "authorData"(id),
    "tags" text [] NOT NULL,
    "source_this" text NOT NULL,
    "source_other" text [] [2] NOT NULL,
    "license_type" VARCHAR(10) NOT NULL,
    "time" TIMESTAMP NOT NULL,
    "active" BOOLEAN NOT NULL
);

insert into
    artworkData (
        "id",
        "title",
        "author_id",
        "author_name",
        "tags",
        "source_this",
        "source_other",
        "license_type",
        "time",
        "active"
    )
VALUES
    (
        'testId',
        'testTitle',
        'testAuthorId',
        'testAuthorName',
        '{"tag1","tag2","tag3"}',
        'example.com/abcd',
        '{{"plat1","example.com/abc"},{"plat2","example.com/def"},{"plat3","example.com/ghi"}}',
        'CC0',
        '2000-01-01 21:00:00',
        false
    );