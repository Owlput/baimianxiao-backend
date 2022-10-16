CREATE TABLE "permitData"(
    "id" BIGSERIAL PRIMARY KEY,
    "source" text [] NOT NULL
);

INSERT INTO
    "permitData"("source")
VALUES
(
        '{
        "permit1.png",
        "permit2.png",
        "permit3.png"
    }'
    );