CREATE TABLE IF NOT EXISTS builds (
	id UUID NOT NULL PRIMARY KEY,
	itemset VARCHAR NOT NULL,
	league VARCHAR NOT NULL,
	required JSONB NOT NULL,
	found JSONB NOT NULL
);
