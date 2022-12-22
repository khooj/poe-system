-- Add down migration script here
DROP TABLE latest_stash;
DROP INDEX account_name_stash_raw_items_index;
DROP TABLE tasks;
DROP TYPE task_type;
DROP TABLE builds;
DROP TABLE raw_items;