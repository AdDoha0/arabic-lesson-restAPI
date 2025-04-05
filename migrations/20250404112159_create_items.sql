

CREATE TABLE textbook
(
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    description TEXT
)


CREATE TABLE lesson
(
    id         SERIAL PRIMARY KEY,
    title      VARCHAR(255) NOT NULL,
    text       TEXT NOT NULL,
    video_url  TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    volume     INTEGER REFERENCES textbook(id)

)


CREATE TABLE word
(
    id          SERIAL PRIMARY KEY,
    term        VARCHAR(100) NOT NULL,
    definition  VARCHAR(100) NOT NULL,
    lesson_id   INTEGER REFERENCES lesson(id)
)



