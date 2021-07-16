DROP INDEX build_info_pob_itemset;
ALTER TABLE build_info DROP pob_file_id;
ALTER TABLE build_info ADD COLUMN pob_url TEXT NOT NULL;
CREATE INDEX build_info_pob_itemset ON build_info(pob_url, itemset);
DROP TABLE pob_file;