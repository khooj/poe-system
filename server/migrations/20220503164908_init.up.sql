-- Add up migration script here
CREATE TABLE IF NOT EXISTS raw_items (
	id VARCHAR NOT NULL,
	account_name VARCHAR NOT NULL,
	stash VARCHAR NOT NULL,
	item JSONB NOT NULL,
	PRIMARY KEY(id)
);
CREATE INDEX account_name_stash_raw_items_index ON raw_items (account_name, stash);
CREATE TABLE IF NOT EXISTS latest_stash (
	stash_id VARCHAR NOT NULL,
	PRIMARY KEY(stash_id)
);
CREATE TYPE task_type AS ENUM ('calculatebuild');
CREATE TABLE IF NOT EXISTS tasks (
	id UUID NOT NULL PRIMARY KEY,
	created_at TIMESTAMPTZ NOT NULL,
	task_type task_type NOT NULL,
	data JSONB NOT NULL
);
CREATE TABLE IF NOT EXISTS builds (
	id UUID NOT NULL PRIMARY KEY,
	itemset VARCHAR NOT NULL,
	league VARCHAR NOT NULL,
	required JSONB NOT NULL,
	found JSONB NOT NULL
);