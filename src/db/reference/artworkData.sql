CREATE TABLE "artworkData"(
    "id" BIGSERIAL PRIMARY KEY,
    "title" VARCHAR(50) NOT NULL,
    "authorId" BIGINT REFERENCES "authorData"(id),
    "permitId" BIGINT REFERENCES "permitData"(id),
    "tags" text [] NOT NULL,
    "sourceOther" text [] [2] NOT NULL,
    "license" VARCHAR(10) NOT NULL,
    "timeOrigin" TIMESTAMP NOT NULL,
    "timeThis" TIMESTAMP NOT NULL,
    "views" INTEGER NOT NULL,
    "active" BOOLEAN NOT NULL
);

INSERT INTO
    "artworkData" (
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
        'testArtworkTitle',
        0,
        0,
        '{"tag1","tag2","tag3"}',
        '{{"plat1","example.com/abc"},{"plat2","example.com/def"},{"plat3","example.com/ghi"}}',
        'CC0',
        '2000-01-01 21:00:00',
        '2000-01-01 21:00:00',
        114,
        false
    );