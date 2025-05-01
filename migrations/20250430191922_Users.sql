-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    email TEXT UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE textbook
(
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    description TEXT
);


CREATE TABLE lesson
(
    id             SERIAL PRIMARY KEY,
    title          VARCHAR(255) NOT NULL,
    text           TEXT NOT NULL,
    video_url      TEXT,
    created_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    textbook_id    INTEGER REFERENCES textbook(id)
);


CREATE TABLE word
(
    id             SERIAL PRIMARY KEY,
    term           VARCHAR(100) NOT NULL,
    definition     VARCHAR(100) NOT NULL,
    lesson_id  INTEGER REFERENCES lesson(id) ON DELETE CASCADE
);

