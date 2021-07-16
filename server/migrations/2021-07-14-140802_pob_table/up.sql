CREATE TABLE IF NOT EXISTS pob_file (
	id TEXT NOT NULL PRIMARY KEY,
	url_token TEXT NOT NULL,
	encoded_pob TEXT NOT NULL,
	UNIQUE(url_token)
);

DROP INDEX build_info_pob_itemset;
ALTER TABLE build_info DROP COLUMN pob_url;
ALTER TABLE build_info ADD pob_file_id TEXT NOT NULL REFERENCES pob_file(id);
CREATE INDEX build_info_pob_itemset ON build_info(pob_file_id, itemset);