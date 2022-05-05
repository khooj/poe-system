CREATE TYPE task_type AS ENUM ('calculatebuild');
CREATE TABLE IF NOT EXISTS tasks (
	id UUID NOT NULL PRIMARY KEY,
	created_at TIMESTAMPTZ NOT NULL,
	task_type task_type NOT NULL,
	data JSONB NOT NULL
);
