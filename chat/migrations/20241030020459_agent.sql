-- Add migration script here
-- modify chats table to add agents column
ALTER TABLE chats
ADD COLUMN agents bigint [] NOT NULL DEFAULT '{}';
-- modify messages table to add modified_content column
ALTER TABLE messages
ADD COLUMN modified_content text;
-- add agent_type type
CREATE TYPE agent_type AS ENUM ('proxy', 'reply', 'tap');
-- add chat_agent table
CREATE TABLE chat_agents (
	id bigserial PRIMARY KEY,
	chat_id bigint NOT NULL REFERENCES chats(id),
	name text NOT NULL UNIQUE,
	type agent_type NOT NULL DEFAULT 'reply',
	prompt text NOT NULL,
	args jsonb NOT NULL,
	created_at timestamptz NOT NULL DEFAULT now(),
	updated_at timestamptz NOT NULL DEFAULT now()
);
