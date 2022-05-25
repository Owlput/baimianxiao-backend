CREATE TABLE "permits"(
    "id" VARCHAR NOT NULL PRIMARY KEY,
    "source" text [] NOT NULL
);

INSERT INTO
    "permits"("id", "source")
VALUES
(
        'testPermitId',
        '{
        "permit1.png",
        "permit2.png",
        "permit3.png"
    }'
    );