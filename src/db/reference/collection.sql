CREATE TABLE "collections"(
    "id" VARCHAR(16) PRIMARY KEY NOT NULL,
    "contents" text[] NOT NULL,
    "tags" text[] NOT NULL,
    "desp" text NOT NULL
);

INSERT INTO "collections"
("id","contents","tags","desp")
VALUES(
    'testCollectionId',
    '{"id1","id2","id3"}',
    '{"tag1","tag2","tag3"}',
    'example description for the collection'
);