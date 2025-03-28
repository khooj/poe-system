TRUNCATE TABLE items;
ALTER TABLE items ADD COLUMN basetype TEXT NOT NULL;
ALTER TABLE items ADD COLUMN category TEXT NOT NULL;
ALTER TABLE items ADD COLUMN subcategory TEXT NOT NULL;

CREATE INDEX IF NOT EXISTS items_cat_idx ON items USING BTREE (category);
CREATE INDEX IF NOT EXISTS items_subcat_idx ON items USING BTREE (subcategory);
