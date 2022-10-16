CREATE TABLE "authorData"(
    "id" BIGSERIAL PRIMARY KEY,
    "name" VARCHAR(50) NOT NULL,
    "contact" text [] [2],
    "recent" text []
);

INSERT INTO
    "authorData" ("name", "contact", "recent")
VALUES
    (
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