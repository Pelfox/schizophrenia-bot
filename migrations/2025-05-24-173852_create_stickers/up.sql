CREATE TABLE IF NOT EXISTS stickers (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    chat_id BIGINT NOT NULL,
    sticker_id TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_stickers_chat_id ON stickers(chat_id);
 