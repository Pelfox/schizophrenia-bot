-- This file should undo anything in `up.sql`

DROP TABLE messages IF EXISTS;

DROP INDEX idx_messages_chat_id IF EXISTS;
