CREATE TABLE "artworkData"(
    "id" VARCHAR(8) NOT NULL PRIMARY KEY,
    "title" VARCHAR(50) NOT NULL,
    "authorId" VARCHAR(8) REFERENCES "authorData"(id),
    "permitId" VARCHAR(8) REFERENCES "permitData"(id),
    "tags" text [] NOT NULL,
    "sourceOther" text [] [2] NOT NULL,
    "license" VARCHAR(10) NOT NULL,
    "timeOrigin" TIMESTAMP NOT NULL,
    "timeThis" TIMESTAMP NOT NULL,
    "views" integer NOT NULL,
    "active" BOOLEAN NOT NULL
);

INSERT INTO
    "artworkData" (
        "id",
        "title",
        "authorId",
        "permitId",
        "tags",
        "sourceOther",
        "license",
        "timeOrigin",
        "timeThis",
        "views",
        "active"
    )
VALUES
    (
        'testArtworkId',
        'testArtworkTitle',
        'testAuthorId',
        'testPermitId',
        '{"tag1","tag2","tag3"}',
        '{{"plat1","example.com/abc"},{"plat2","example.com/def"},{"plat3","example.com/ghi"}}',
        'CC0',
        '2000-01-01 21:00:00',
        '2000-01-01 21:00:00',
        114,
        false
    );