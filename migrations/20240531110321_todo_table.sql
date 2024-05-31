-- Add migration script here
CREATE TABLE todos
(
    id          SERIAL PRIMARY KEY,
    user_id     INTEGER NOT NULL,
    title       VARCHAR(255) NOT NULL,
    description TEXT,
    completed   BOOLEAN DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX todos_user_id_ix
    ON todos (user_id);

ALTER TABLE todos
    ADD CONSTRAINT todos_user_id_title_uq
        UNIQUE (user_id, title);