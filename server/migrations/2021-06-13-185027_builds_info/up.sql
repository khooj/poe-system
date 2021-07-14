CREATE TABLE IF NOT EXISTS build_info (
	id TEXT PRIMARY KEY NOT NULL,
	pob_url TEXT NOT NULL,
	itemset TEXT NOT NULL
);
CREATE INDEX build_info_pob_itemset ON build_info(pob_url, itemset);
CREATE TABLE IF NOT EXISTS builds_match (
	id TEXT NOT NULL,
	idx INTEGER NOT NULL,
	score INTEGER NOT NULL,
	item_id TEXT NOT NULL,
	PRIMARY KEY(id, idx),
	FOREIGN KEY(id) REFERENCES build_info(id) ON DELETE CASCADE,
	FOREIGN KEY(item_id) REFERENCES items(id) ON DELETE CASCADE
);