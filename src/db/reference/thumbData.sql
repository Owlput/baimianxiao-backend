CREATE TABLE "thumbData"(
    "artworkId" BIGSERIAL PRIMARY KEY,
    "artworkTitle" VARCHAR(50) NOT NULL,
    "authorName" VARCHAR(50) NOT NULL,
    "authorId" VARCHAR(8) NOT NULL,
    "time" TIMESTAMP NOT NULL,
    "views" integer NOT NULL,
    "active" BOOLEAN NOT NULL
);

INSERT INTO
    "thumbData" (
        "artworkTitle",
        "authorId",
        "authorName",
        "time",
        "views",
        "active"
    )
VALUES
    (
        'testArtworkTitle',
        'testAuthorId',
        'testAuthorName',
        '2000-01-01 22:00:00',
        5,
        true
    );