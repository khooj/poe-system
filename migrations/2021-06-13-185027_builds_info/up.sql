CREATE TABLE IF NOT EXISTS build_info (
	id TEXT PRIMARY KEY NOT NULL,
	pob_url TEXT NOT NULL,
	itemset TEXT NOT NULL
);
CREATE INDEX build_info_pob_itemset ON build_info(pob_url, itemset);