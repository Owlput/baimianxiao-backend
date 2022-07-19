CREATE TABLE "permitData"(
    "id" VARCHAR(8) NOT NULL PRIMARY KEY,
    "source" text [] NOT NULL
);

INSERT INTO
    "permitData"("id", "source")
VALUES
(
        'testPermitId',
        '{
        "permit1.png",
        "permit2.png",
        "permit3.png"
    }'
    );