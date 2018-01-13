-- Your SQL goes here

CREATE TABLE users (
    uuid UUID PRIMARY KEY,
    login TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    created TIMESTAMP NOT NULL,
    updated TIMESTAMP NOT NULL
);

CREATE TYPE SourceType AS ENUM (
    'Rss', 'Twitter'
);

CREATE TABLE sources (
    uuid UUID PRIMARY KEY,
    source_type SourceType NOT NULL,
    data JSONB,
    error TEXT,
    created TIMESTAMP,
    updated TIMESTAMP
);

CREATE TABLE bookmarks (
    uuid UUID PRIMARY KEY,
    url TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    path TEXT,
    created TIMESTAMP NOT NULL,
    updated TIMESTAMP NOT NULL,
    user_uuid UUID NOT NULL REFERENCES users(uuid)
);

CREATE TABLE conversations (
    uuid UUID PRIMARY KEY,
    level INT NOT NULL,
    created TIMESTAMP NOT NULL,
    updated TIMESTAMP NOT NULL
);
