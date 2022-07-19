CREATE TABLE "authorData"(
    "id" VARCHAR(8) PRIMARY KEY NOT NULL,
    "name" VARCHAR(50) NOT NULL,
    "contact" text [] [2],
    "recent" text []
);

INSERT INTO
    "authorData" ("id", "name", "contact", "recent")
VALUES
    (
        'testAuthorId',
        'testAuthorName',
        '{ { "plat1",
        "example.com/abc" },
        { "plat2",
        "example.com/cde" },
        { "plat3",
        "example.com/fgh" } }',
        '{ "testId1",
        "testId2",
        "testId3" }'
    );