-- Add down migration script here
DROP INDEX account_name_stash_raw_items_index;
DROP TABLE raw_items;