CREATE TABLE thumb_data(
    "id" VARCHAR(16) NOT NULL PRIMARY KEY,
    "title" VARCHAR(50) NOT NULL,
    "authorName" VARCHAR(50) NOT NULL,
    "authorImg" VARCHAR(25) NOT NULL,
    "time" TIMESTAMP NOT NULL,
    "views" integer NOT NULL,
    "active" BOOLEAN NOT NULL
);

INSERT INTO
    thumb_data (
        "id",
        "title",
        "authorName",
        "authorImg",
        "time",
        "views",
        "active"
    )
VALUES
    (
        'testId',
        'testTitle',
        'testName',
        'test.file',
        '2000-01-01 22:00:00',
        5,
        true
    );