CREATE TABLE "collectionData"(
    "id" VARCHAR(8) PRIMARY KEY NOT NULL,
    "contents" text [] NOT NULL,
    "tags" text [] NOT NULL,
    "desp" text NOT NULL
);

INSERT INTO
    "collectionData" ("id", "contents", "tags", "desp")
VALUES
(
        'testCollectionId',
        '{"id1","id2","id3"}',
        '{"tag1","tag2","tag3"}',
        'example description for the collection'
    );