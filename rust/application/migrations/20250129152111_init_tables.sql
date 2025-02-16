CREATE TABLE IF NOT EXISTS stashes(
  id TEXT NOT NULL,
  item_id TEXT NOT NULL,
  PRIMARY KEY(id, item_id)
);

CREATE TABLE IF NOT EXISTS items(
  id TEXT PRIMARY KEY,
  data JSONB NOT NULL
);

CREATE INDEX IF NOT EXISTS items_mods_idx ON items USING GIN ((data -> 'mods'));

CREATE TABLE IF NOT EXISTS latest_stash(
  id TEXT PRIMARY KEY
);
