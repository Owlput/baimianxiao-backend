CREATE TABLE "collectionData"(
    "id" BIGSERIAL PRIMARY KEY,
    "contents" text [] NOT NULL,
    "tags" text [] NOT NULL,
    "desp" text NOT NULL
);

INSERT INTO
    "collectionData" ("contents", "tags", "desp")
VALUES
(
        '{"id1","id2","id3"}',
        '{"tag1","tag2","tag3"}',
        'example description for the collection'
    );