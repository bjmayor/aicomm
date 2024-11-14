-- add adapter and model to chat_agents
-- define adapter_type enum
CREATE TYPE adapter_type AS ENUM ('ollama', 'openai');
ALTER TABLE chat_agents
ADD COLUMN adapter adapter_type NOT NULL DEFAULT 'ollama',
	ADD COLUMN model varchar(255) NOT NULL DEFAULT 'llama3.2';
